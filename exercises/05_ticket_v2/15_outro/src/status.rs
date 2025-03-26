// Done: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug)]
pub struct StatusParseError {
    invalid_status: String,
}

impl std::fmt::Display for StatusParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "`{}` is not a valid status. Use one of: ToDo, InProgress, Done",
            self.invalid_status
        )
    }
}

impl TryFrom<String> for Status {
    type Error = StatusParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        match value.as_str() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(StatusParseError {
                invalid_status: value,
            }),
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = StatusParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Status::try_from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
