use serde::Serialize;

#[derive(Debug , Serialize)]
struct UserProfile {
    display_name: String,
}

#[allow(dead_code)]
impl UserProfile {
    fn display_name(&self) -> String {
        self.display_name.clone()
    }

    fn set_display_name(&mut self, display_name: String) {
        self.display_name = display_name
    }
}

#[derive(Debug , Serialize)] 
struct UserLocation {
    country_code: String,
    timezone: String,
    
}

#[allow(dead_code)]
impl UserLocation {
    fn country_code(&self) -> &str {
        &self.country_code
    }

    fn timezone(&self) -> &str {
        &self.timezone
    }

    fn set_country_code(&mut self, country_code: String) {
        self.country_code = country_code;
    }

    fn set_timezone(&mut self, timezone: String) {
        self.timezone = timezone;
    }
}
#[derive(Debug ,Serialize)]
struct UserRoles {
    roles: Vec<String>,
}

#[allow(dead_code)]
impl UserRoles {
    fn all(&self) -> &Vec<String> {
        &self.roles
    }

    fn add(&mut self, role: String) {
        self.roles.push(role);
    }
}


#[derive(Debug , Serialize)]
struct UserPresence {
    status: String,
}

#[allow(dead_code)]
impl UserPresence {
    fn status(&self) -> &str {
        &self.status
    }

    fn set_status(&mut self, status: String) {
        self.status = status;
    }
}
#[derive(Debug , Serialize)]
struct User {
    id: u64,
    username: String,
    profile: UserProfile,
    location: UserLocation,
    roles: UserRoles,
    presence: UserPresence,
}

#[allow(dead_code)]
impl User {
    // Getters

    fn id(&self) -> u64 {
        self.id
    }

    fn username(&self) -> String {
        self.username.clone()
    }

    fn display_name(&self) -> String {
        self.profile.display_name.clone()
    }

    fn country_code(&self) -> String {
        self.location.country_code.clone()
    }

    fn timezone(&self) -> String {
        self.location.timezone.clone()
    }

    fn roles(&self) -> &Vec<String> {
        &self.roles.roles
    }

    fn status(&self) -> String {
        self.presence.status.clone()
    }

    fn set_username(&mut self, username: String) {
        self.username = username;
    }

    fn set_display_name(&mut self, display_name: String) {
        self.profile.display_name = display_name;
    }

    fn set_country_code(&mut self, country_code: String) {
        self.location.country_code = country_code;
    }

    fn set_timezone(&mut self, timezone: String) {
        self.location.timezone = timezone;
    }

    fn set_status(&mut self, status: String) {
        self.presence.status = status;
    }

    fn add_role(&mut self, role: String) {
        self.roles.roles.push(role);
    }

    fn get_user_info(&self) -> String {
        format!(
            "ID: {}, Username: {}, Display Name: {}, Country: {}, Timezone: {}, Roles: {:?}, Status: {}",
            self.id,
            self.username,
            self.profile.display_name,
            self.location.country_code,
            self.location.timezone,
            self.roles.roles,
            self.presence.status
        )
    }

    // Uses values passed as arguments
    fn get_user_info_with_args(
        &self,
        id: u64,
        username: &str,
        display_name: &str,
        country_code: &str,
        timezone: &str,
        roles: &[String],
        status: &str,
    ) -> String {
        format!(
            "ID: {}, Username: {}, Display Name: {}, Country: {}, Timezone: {}, Roles: {:?}, Status: {}",
            id,
            username,
            display_name,
            country_code,
            timezone,
            roles,
            status
        )
    }
}


fn main() {
    let  user = User {
        id: 1,
        username: "shivam".to_string(),
        profile: UserProfile {
            display_name: "shivamsinh".to_string(),
        },
        location: UserLocation {
            country_code: "IN".to_string(),
            timezone: "Asia/Kolkata".to_string(),
        },
        roles: UserRoles {
            roles: vec!["user".to_string()],
        },

        presence: UserPresence {
            status: "online".to_string(),
        },
    };
    
    println!("{:#?}", user);

    // conver struct -> JSON (Serializetion)

    let json_user = serde_json::to_string_pretty(&user).unwrap();
    println!("{}",json_user)


}
