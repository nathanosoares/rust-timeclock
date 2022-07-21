use super::session::Session;
use chrono::{Date, DateTime, Utc};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum WorkdayError {
    #[error("An open session already exists.")]
    OpenSessionAlreadyExists,

    #[error("One or more sessions in range.")]
    OverlappingSessions,

    #[error("Sessions is empty.")]
    EmptySessions,

    #[error("The current session already ended.")]
    CurrentSessionAlreadyEnded,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Workday {
    date: Date<Utc>,
    sessions: Vec<Session>,
}

impl Workday {
    pub fn new(date: Date<Utc>) -> Self {
        Self {
            date,
            sessions: Vec::new(),
        }
    }

    pub fn date(&self) -> Date<Utc> {
        self.date
    }

    pub fn add_session(&mut self, session: Session) -> anyhow::Result<(), WorkdayError> {
        let last_session = self.sessions.last();

        if let Some(last) = last_session {
            if last.ended_at().is_none() && session.ended_at().is_none() {
                return Err(WorkdayError::OpenSessionAlreadyExists);
            }
        }

        let sessions_in_range = self.sessions_in_range(session.started_at(), session.ended_at());

        if !sessions_in_range.is_empty() {
            return Err(WorkdayError::OverlappingSessions);
        }

        self.sessions.push(session);
        self.sessions.sort();

        Ok(())
    }

    pub fn sessions(&self) -> Vec<Session> {
        self.sessions.clone()
    }

    pub fn sessions_in_range(
        &self,
        start: DateTime<Utc>,
        end: Option<DateTime<Utc>>,
    ) -> Vec<Session> {
        self.sessions
            .clone()
            .into_iter()
            .filter(|session| {
                if start.ge(&session.started_at()) {
                    if session.ended_at().is_none() {
                        return true;
                    }

                    if start.le(&session.ended_at().unwrap()) {
                        return true;
                    }
                }

                if start.le(&session.started_at()) {
                    if end.is_none() {
                        return true;
                    }

                    if end.unwrap().ge(&session.started_at()) {
                        return true;
                    }
                }

                false
            })
            .collect()
    }

    pub fn end_current_session(&mut self, ended_at: DateTime<Utc>) -> Result<(), WorkdayError> {
        let last_session = self.sessions.last();

        if last_session.is_none() {
            return Err(WorkdayError::EmptySessions);
        }

        if last_session.unwrap().ended_at().is_some() {
            return Err(WorkdayError::CurrentSessionAlreadyEnded);
        }

        let started_at = last_session.unwrap().started_at();

        self.sessions.pop();

        self.add_session(Session::new(started_at, Some(ended_at)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn add_new_a_session() -> anyhow::Result<()> {
        let mut workday = Workday::new(Utc.ymd(2022, 07, 01));
        let started_at = Utc.ymd(2022, 07, 01).and_hms(8, 0, 0);
        let ended_at = Utc.ymd(2022, 07, 01).and_hms(12, 0, 0);

        workday.add_session(Session::new(started_at, Some(ended_at)))?;

        let sessions = workday.sessions();

        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].started_at(), started_at);
        assert_eq!(sessions[0].ended_at().unwrap(), ended_at);

        Ok(())
    }

    #[test]
    fn find_all_sessions_in_range() -> anyhow::Result<()> {
        let mut workday = Workday::new(Utc.ymd(2022, 07, 01));

        workday
            .add_session(Session::new(
                Utc.ymd(2022, 07, 01).and_hms(8, 0, 0),
                Some(Utc.ymd(2022, 07, 01).and_hms(9, 30, 0)),
            ))
            .unwrap();

        workday
            .add_session(Session::new(
                Utc.ymd(2022, 07, 01).and_hms(10, 31, 0),
                Some(Utc.ymd(2022, 07, 01).and_hms(12, 30, 0)),
            ))
            .unwrap();

        workday
            .add_session(Session::new(
                Utc.ymd(2022, 07, 01).and_hms(13, 30, 0),
                Some(Utc.ymd(2022, 07, 01).and_hms(18, 00, 0)),
            ))
            .unwrap();

        let sessions_in_range = workday.sessions_in_range(
            Utc.ymd(2022, 07, 01).and_hms(10, 40, 0),
            Some(Utc.ymd(2022, 07, 01).and_hms(17, 00, 0)),
        );

        assert_eq!(sessions_in_range.len(), 2);

        let sessions_in_range = workday.sessions_in_range(
            Utc.ymd(2022, 07, 01).and_hms(17, 40, 0),
            Some(Utc.ymd(2022, 07, 01).and_hms(23, 00, 0)),
        );

        assert_eq!(sessions_in_range.len(), 1);

        Ok(())
    }

    #[test]
    fn expect_an_error_when_add_sessions_with_time_overlap() -> anyhow::Result<()> {
        let mut workday = Workday::new(Utc.ymd(2022, 07, 01));

        workday.add_session(Session::new(
            Utc.ymd(2022, 07, 01).and_hms(8, 0, 0),
            Some(Utc.ymd(2022, 07, 01).and_hms(9, 30, 0)),
        ))?;

        let result = workday.add_session(Session::new(
            Utc.ymd(2022, 07, 01).and_hms(7, 31, 0),
            Some(Utc.ymd(2022, 07, 01).and_hms(12, 30, 0)),
        ));

        assert!(result.is_err());
        assert_eq!(WorkdayError::OverlappingSessions, result.err().unwrap());

        workday.add_session(Session::new(Utc.ymd(2022, 07, 01).and_hms(9, 31, 0), None))?;

        let result =
            workday.add_session(Session::new(Utc.ymd(2022, 07, 01).and_hms(12, 30, 0), None));

        assert!(result.is_err());
        assert_eq!(
            WorkdayError::OpenSessionAlreadyExists,
            result.err().unwrap()
        );

        Ok(())
    }
}
