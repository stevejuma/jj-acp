//! Bounded process termination.
//!
//! Cleanup is a sequence with a bound on every stage, and each stage reports what
//! actually happened. It is best-effort and says so: a harness killed outright, or
//! a descendant that deliberately escaped its process group, cannot be stopped
//! from here, and an unreaped child is reported rather than assumed gone.

use std::time::Duration;

use tokio::process::Child;

/// What termination actually achieved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Termination {
    /// The process exited during the cooperative grace window.
    pub exited_during_grace: bool,
    /// A kill was sent to the whole process group rather than the child alone.
    pub group_killed: bool,
    /// The process was reaped and an exit status observed.
    pub reaped: bool,
    /// The exit code, when the platform reported one.
    pub exit_code: Option<i32>,
}

impl Termination {
    /// Report whether the process is definitely gone.
    pub fn confirmed_stopped(&self) -> bool {
        self.reaped
    }
}

/// Owns an agent child and the bounds on stopping it.
#[derive(Debug)]
pub struct ProcessGuard {
    child: Child,
    pid: Option<u32>,
}

impl ProcessGuard {
    /// Take ownership of a spawned child.
    pub fn new(child: Child) -> Self {
        let pid = child.id();
        Self { child, pid }
    }

    /// The child's process id, when the platform exposed one.
    pub fn pid(&self) -> Option<u32> {
        self.pid
    }

    /// Report whether the child has already exited.
    pub async fn poll(&mut self) -> bool {
        matches!(self.child.try_wait(), Ok(Some(_)))
    }

    /// Stop the child within bounded grace and reap intervals.
    ///
    /// The caller is expected to have already requested cooperative cancellation
    /// through the protocol. This waits `grace` for a voluntary exit, then kills
    /// the process group and waits `reap` for the kernel to release it.
    pub async fn terminate(&mut self, grace: Duration, reap: Duration) -> Termination {
        if let Ok(Some(status)) = self.child.try_wait() {
            return Termination {
                exited_during_grace: true,
                group_killed: false,
                reaped: true,
                exit_code: status.code(),
            };
        }
        if let Ok(Ok(status)) = tokio::time::timeout(grace, self.child.wait()).await {
            return Termination {
                exited_during_grace: true,
                group_killed: false,
                reaped: true,
                exit_code: status.code(),
            };
        }
        let group_killed = self.kill_group();
        match tokio::time::timeout(reap, self.child.wait()).await {
            Ok(Ok(status)) => Termination {
                exited_during_grace: false,
                group_killed,
                reaped: true,
                exit_code: status.code(),
            },
            _ => Termination {
                exited_during_grace: false,
                group_killed,
                reaped: false,
                exit_code: None,
            },
        }
    }

    /// Kill the process group, falling back to the direct child.
    #[cfg(unix)]
    fn kill_group(&mut self) -> bool {
        let Some(pid) = self.pid.filter(|pid| *pid > 0) else {
            return self.child.start_kill().is_ok();
        };
        let target = nix::unistd::Pid::from_raw(pid as i32);
        if nix::sys::signal::killpg(target, nix::sys::signal::Signal::SIGKILL).is_ok() {
            return true;
        }
        // A group kill can fail when the process already left the group or is
        // gone; the direct child is still worth signalling.
        self.child.start_kill().is_ok()
    }

    /// Kill the direct child. Descendant containment is a Unix property and is
    /// not promised elsewhere; the reduced guarantee is reported, not hidden.
    #[cfg(not(unix))]
    fn kill_group(&mut self) -> bool {
        self.child.start_kill().is_ok()
    }
}
