use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct PropertyError;
#[derive(Debug, Clone)]
pub struct IDError;
#[derive(Debug, Clone)]
pub struct TitleError;
#[derive(Debug, Clone)]
pub struct DataValidError;

impl fmt::Display for PropertyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "you cannot delete something that isn't existing. Wrong identifier.")
    }
}

impl fmt::Display for IDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There was an Error with the ID you used.")
    }
}

impl fmt::Display for TitleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There was an Error with the Title you used.")
    }
}

impl fmt::Display for DataValidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There was an Error with the Validity of the Database.")
    }
}

impl error::Error for PropertyError {
    fn description(&self) -> &str {
        "you cannot delete something that isn't existing. Wrong identifier."
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl error::Error for IDError {
    fn description(&self) -> &str {
        "There was an Error with the ID you used."
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl error::Error for TitleError {
    fn description(&self) -> &str {
        "There was an Error with the Title you used."
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl error::Error for DataValidError {
    fn description(&self) -> &str {
        "There is something wrong with the Data in the Database."
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
