use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomerGender {
    Female,
    Male,
    Other
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmploymentDetail {
    employer_name: Option<String>,
    nature_of_business: Option<String>,
    role_description: Option<String>
}
impl EmploymentDetail {
    pub fn new() -> Self {
        EmploymentDetail {
            employer_name: None,
            nature_of_business: None,
            role_description: None
        }
    }
    pub fn set_employer_name(&mut self, employer_name: String) -> &mut Self {
        self.employer_name = Some(employer_name);
        self
    }
    pub fn get_employer_name(&self) -> Option<&String> {
        self.employer_name.as_ref()
    }
    pub fn set_nature_of_business(&mut self, nature_of_business: String) -> &mut Self {
        self.nature_of_business = Some(nature_of_business);
        self
    }
    pub fn get_nature_of_business(&self) -> Option<&String> {
        self.nature_of_business.as_ref()
    }
    pub fn set_role_description(&mut self, role_description: String) -> &mut Self {
        self.role_description = Some(role_description);
        self
    }
    pub fn get_role_description(&self) -> Option<&String> {
        self.role_description.as_ref()
    }
    pub fn build(&mut self) -> EmploymentDetail {
        self.clone()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct IndividualDetail {
    given_names: String,
    surname: Option<String>,
    nationality: Option<String>,
    place_of_birth: Option<String>,
    date_of_birth: Option<NaiveDate>,
    gender: Option<CustomerGender>,
    employment: Option<EmploymentDetail>
}
impl IndividualDetail {
    pub fn new(given_names: String) -> Self {
        IndividualDetail {
            given_names,
            surname: None,
            nationality: None,
            place_of_birth: None,
            date_of_birth: None,
            gender: None,
            employment: None
        }
    }
    pub fn get_given_names(&self) -> &String {
        &self.given_names
    }
    pub fn set_surname(&mut self, surname: String) -> &mut Self {
        self.surname = Some(surname);
        self
    }
    pub fn get_surname(&self) -> Option<&String> {
        self.surname.as_ref()
    }
    pub fn set_nationality(&mut self, nationality: String) -> &mut Self {
        self.nationality = Some(nationality);
        self
    }
    pub fn get_nationality(&self) -> Option<&String> {
        self.nationality.as_ref()
    }
    pub fn set_place_of_birth(&mut self, place_of_birth: String) -> &mut Self {
        self.place_of_birth = Some(place_of_birth);
        self
    }
    pub fn get_place_of_birth(&self) -> Option<&String> {
        self.place_of_birth.as_ref()
    }
    pub fn set_date_of_birth(&mut self, date_of_birth: &str) -> &mut Self {
        self.date_of_birth = Some(NaiveDate::parse_from_str(date_of_birth, "%Y-%m-%d").unwrap());
        self
    }
    pub fn get_date_of_birth(&self) -> Option<&NaiveDate> {
        self.date_of_birth.as_ref()
    }
    pub fn set_gender(&mut self, gender: CustomerGender) -> &mut Self {
        self.gender = Some(gender);
        self
    }
    pub fn get_gender(&self) -> Option<&CustomerGender> {
        self.gender.as_ref()
    }
    pub fn set_employment(&mut self, employment: EmploymentDetail) -> &mut Self {
        self.employment = Some(employment);
        self
    }
    pub fn get_employment(&self) -> Option<&EmploymentDetail> {
        self.employment.as_ref()
    }
    pub fn build(&mut self) -> IndividualDetail {
        self.clone()
    }
}