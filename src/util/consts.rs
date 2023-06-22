#[derive(Debug, PartialEq)]
pub struct RoleOption {
  pub id: String,
  pub name: String,
  pub addi_roles: Option<Vec<String>>,
}

impl RoleOption {
  pub fn new(id: &str, name: &str, addi_roles: Option<Vec<&'static str>>) -> Self {
    let addi_roles = if let Some(role) = addi_roles {
      let u: Vec<_> = role.iter().map(|r| r.to_string()).collect();
      Some(u)
    } else {
      None
    };

    Self {
      id: id.to_string(),
      name: name.to_string(),
      addi_roles,
    }
  }
}

#[allow(unused)]
pub fn get_jets() -> Vec<RoleOption> {
  let mut models: Vec<RoleOption> = Vec::new();
  models.push(RoleOption::new(
    "a10c",
    "Pilot A-10C Warthog",
    Some(vec!["Modern"]),
  ));
  models.push(RoleOption::new(
    "ajs37",
    "Pilot AJS-37 Viggen",
    Some(vec!["Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "av8b",
    "Pilot AV-8B Night Attack V/STOL",
    Some(vec!["Modern"]),
  ));
  models.push(RoleOption::new(
    "c101",
    "Pilot C-101 Aviojet",
    Some(vec!["Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "f18c",
    "Pilot F/A-18C Hornet",
    Some(vec!["Modern"]),
  ));
  models.push(RoleOption::new(
    "f14p",
    "Pilot F-14 Tomcat Pilot",
    Some(vec!["Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "f14r",
    "Pilot F-14 Tomcat RIO",
    Some(vec!["Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "f15e-p",
    "Pilot F-15E Strike Eagle Pilot",
    Some(vec!["Modern"]),
  ));
  models.push(RoleOption::new(
    "f15e-w",
    "Pilot F-15E Strike Eagle WSO",
    Some(vec!["Modern"]),
  ));
  models.push(RoleOption::new(
    "f16c",
    "Pilot F-16C Viper",
    Some(vec!["Modern"]),
  ));
  models.push(RoleOption::new(
    "f5",
    "Pilot F-5E Tiger II",
    Some(vec!["Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "f86",
    "Pilot F-86F Sabre",
    Some(vec!["Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "jf17",
    "Pilot JF-17 Thunder",
    Some(vec!["Modern"]),
  ));
  models.push(RoleOption::new(
    "l39",
    "Pilot L-39 Albatros",
    Some(vec!["Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "mb339",
    "Pilot MB-339",
    Some(vec!["Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "m2000",
    "Pilot M-2000C",
    Some(vec!["Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "mig15",
    "Pilot MiG-15bis",
    Some(vec!["Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "mig19",
    "Pilot MiG-19P Farmer",
    Some(vec!["Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "mig21",
    "Pilot MiG-21bis",
    Some(vec!["Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "f1",
    "Pilot Mirage F1",
    Some(vec!["Kalter Krieg"]),
  ));

  models
}

#[allow(unused)]
pub fn get_fc() -> Vec<RoleOption> {
  let mut models: Vec<RoleOption> = Vec::new();
  models.push(RoleOption::new(
    "a10a",
    "Pilot A-10A",
    Some(vec!["Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "f15c",
    "Pilot F-15C Eagle",
    Some(vec!["Modern"]),
  ));
  models.push(RoleOption::new(
    "mig29",
    "Pilot MiG-29",
    Some(vec!["Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "su25a",
    "Su-25A",
    Some(vec!["Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "su25t",
    "Pilot Su-25T",
    Some(vec!["Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "su27",
    "Pilot Su-27",
    Some(vec!["Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "su33",
    "Pilot Su-33",
    Some(vec!["Kalter Krieg"]),
  ));

  models
}

#[allow(unused)]
pub fn get_helis() -> Vec<RoleOption> {
  let mut models: Vec<RoleOption> = Vec::new();
  models.push(RoleOption::new(
    "ah64d",
    "Pilot AH-64D",
    Some(vec!["Hubschrauber", "Modern"]),
  ));
  models.push(RoleOption::new(
    "ka50",
    "Pilot Black Shark",
    Some(vec!["Hubschrauber", "Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "mi24",
    "Pilot Mi-24P Hind",
    Some(vec!["Hubschrauber", "Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "mi8",
    "Pilot Mi-8MTV2 Magnificent Eight",
    Some(vec!["Hubschrauber", "Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "sa342",
    "Pilot SA342 Gazelle",
    Some(vec!["Hubschrauber", "Kalter Krieg"]),
  ));
  models.push(RoleOption::new(
    "uh1h",
    "Pilot UH-1H Huey",
    Some(vec!["Hubschrauber", "Kalter Krieg"]),
  ));

  models
}

#[allow(unused)]
pub fn get_props() -> Vec<RoleOption> {
  let mut models: Vec<RoleOption> = Vec::new();
  models.push(RoleOption::new(
    "bf109",
    "Pilot Bf 109 K-4 Kurfürst",
    Some(vec!["WWII"]),
  ));
  models.push(RoleOption::new(
    "ce2",
    "Pilot Christen Eagle II",
    Some(vec!["Modern"]),
  ));
  models.push(RoleOption::new(
    "fw190a8",
    "Pilot Fw 190 A-8 Anton",
    Some(vec!["WWII"]),
  ));
  models.push(RoleOption::new(
    "fw190d9",
    "Pilot Fw 190 D-9 Dora",
    Some(vec!["WWII"]),
  ));
  models.push(RoleOption::new("i16", "Pilot I-16", Some(vec!["WWII"])));
  models.push(RoleOption::new(
    "mosq",
    "Pilot Mosquito FB VI",
    Some(vec!["WWII"]),
  ));
  models.push(RoleOption::new(
    "p47",
    "Pilot P-47D Thunderbolt",
    Some(vec!["WWII"]),
  ));
  models.push(RoleOption::new(
    "p51",
    "Pilot P-51D Mustang",
    Some(vec!["WWII"]),
  ));
  models.push(RoleOption::new(
    "spitfire",
    "Pilot Spitfire LF Mk. IX",
    Some(vec!["WWII"]),
  ));
  models.push(RoleOption::new("tf51", "Pilot TF-51D", Some(vec!["WWII"])));
  models.push(RoleOption::new(
    "yak52",
    "Pilot Yak-52",
    Some(vec!["Kalter Krieg"]),
  ));

  models
}

#[allow(unused)]
pub fn get_other() -> Vec<RoleOption> {
  let mut roles: Vec<RoleOption> = Vec::new();
  roles.push(RoleOption::new("BuddySpike", "BuddySpike", None));
  roles.push(RoleOption::new("Cockpitbauer", "Cockpitbauer", None));
  roles.push(RoleOption::new("Liberation", "Liberation", None));
  roles.push(RoleOption::new("PVP", "PVP", None));
  roles.push(RoleOption::new("Rotorheads", "Rotorheads", None));
  roles.push(RoleOption::new("RotorOps", "RotorOps", None));
  roles.push(RoleOption::new("VirtualReality", "Virtual Reality", None));

  roles
}

#[allow(unused)]
pub fn get_all() -> Vec<RoleOption> {
  let mut models: Vec<RoleOption> = Vec::new();
  models.extend(get_jets());
  models.extend(get_helis());
  models.extend(get_props());
  models.extend(get_fc());

  models
}
