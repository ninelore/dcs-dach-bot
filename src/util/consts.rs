#[derive(PartialEq)]
pub struct RoleOption {
  pub id: String,
  pub name: String,
  pub add_role_names: Option<Vec<String>>
}

fn build_role_option(id: String, name: String, add_roles: Option<Vec<String>>) -> RoleOption {
  RoleOption { id: id, name: name, add_role_names: add_roles }
}

#[allow(unused)]
pub fn get_jets() -> Vec<RoleOption> {

  let mut models: Vec<RoleOption> = Vec::new();
  models.push(build_role_option("a10c".to_string(), "Pilot A-10C Warthog".to_string(), Some(vec!["Modern".to_string()])));
  models.push(build_role_option("ajs37".to_string(), "Pilot AJS-37 Viggen".to_string(), Some(vec!["Kalter Krieg".to_string()])));
  models.push(build_role_option("av8b".to_string(), "Pilot AV-8B Night Attack V/STOL".to_string(), Some(vec!["Modern".to_string()])));
  models.push(build_role_option("c101".to_string(), "Pilot C-101 Aviojet".to_string(), Some(vec!["Kalter Krieg".to_string()])));
  models.push(build_role_option("f18c".to_string(), "Pilot F/A-18C Hornet".to_string(), Some(vec!["Modern".to_string()])));
  models.push(build_role_option("f14p".to_string(), "Pilot F-14 Tomcat Pilot".to_string(), Some(vec!["Kalter Krieg".to_string()])));
  models.push(build_role_option("f14r".to_string(), "Pilot F-14 Tomcat RIO".to_string(), Some(vec!["Kalter Krieg".to_string()])));
  models.push(build_role_option("f16c".to_string(), "Pilot F-16C Viper".to_string(), Some(vec!["Modern".to_string()])));
  models.push(build_role_option("f5".to_string(), "Pilot F-5E Tiger II".to_string(), Some(vec!["Kalter Krieg".to_string()])));
  models.push(build_role_option("f86".to_string(), "Pilot F-86F Sabre".to_string(), Some(vec!["Kalter Krieg".to_string()])));
  models.push(build_role_option("jf17".to_string(), "Pilot JF-17 Thunder".to_string(), Some(vec!["Modern".to_string()])));
  models.push(build_role_option("l39".to_string(), "Pilot L-39 Albatros".to_string(), Some(vec!["Kalter Krieg".to_string()])));
  models.push(build_role_option("m2000".to_string(), "Pilot M-2000C".to_string(), Some(vec!["Kalter Krieg".to_string()])));
  models.push(build_role_option("mig15".to_string(), "Pilot MiG-15bis".to_string(), Some(vec!["Kalter Krieg".to_string()])));
  models.push(build_role_option("mig19".to_string(), "Pilot MiG-19P Farmer".to_string(), Some(vec!["Kalter Krieg".to_string()])));
  models.push(build_role_option("mig21".to_string(), "Pilot MiG-21bis".to_string(), Some(vec!["Kalter Krieg".to_string()])));
  models.push(build_role_option("f1".to_string(), "Pilot Mirage F1".to_string(), Some(vec!["Kalter Krieg".to_string()])));

  return models;

}

#[allow(unused)]
pub fn get_fc() -> Vec<RoleOption> {

  let mut models: Vec<RoleOption> = Vec::new();
  models.push(build_role_option("a10a".to_string(), "Pilot A-10A".to_string(), Some(vec!["Kalter Krieg".to_string()])));
  models.push(build_role_option("f15c".to_string(), "Pilot F-15C Eagle".to_string(), Some(vec!["Modern".to_string()])));
  models.push(build_role_option("mig29".to_string(), "Pilot MiG-29".to_string(), Some(vec!["Kalter Krieg".to_string()])));
  models.push(build_role_option("su25a".to_string(), "Pilot Su-25A".to_string(), Some(vec!["Kalter Krieg".to_string()])));
  models.push(build_role_option("su25t".to_string(), "Pilot Su-25T".to_string(), Some(vec!["Kalter Krieg".to_string()])));
  models.push(build_role_option("su27".to_string(), "Pilot Su-27".to_string(), Some(vec!["Kalter Krieg".to_string()])));
  models.push(build_role_option("su33".to_string(), "Pilot Su-33".to_string(), Some(vec!["Kalter Krieg".to_string()])));

  return models;

}

#[allow(unused)]
pub fn get_helis() -> Vec<RoleOption> {

  let mut models: Vec<RoleOption> = Vec::new();
  models.push(build_role_option("ah64d".to_string(), "Pilot AH-64D".to_string(), Some(vec!["Hubschrauber".to_string(), "Modern".to_string()])));
  models.push(build_role_option("ka50".to_string(), "Pilot Black Shark".to_string(), Some(vec!["Hubschrauber".to_string(), "Kalter Krieg".to_string()])));
  models.push(build_role_option("mi24".to_string(), "Pilot Mi-24P Hind".to_string(), Some(vec!["Hubschrauber".to_string(), "Kalter Krieg".to_string()])));
  models.push(build_role_option("mi8".to_string(), "Pilot Mi-8MTV2 Magnificent Eight".to_string(), Some(vec!["Hubschrauber".to_string(), "Kalter Krieg".to_string()])));
  models.push(build_role_option("uh1h".to_string(), "Pilot UH-1H Huey".to_string(), Some(vec!["Hubschrauber".to_string(), "Kalter Krieg".to_string()])));
  models.push(build_role_option("sa342".to_string(), "Pilot SA342 Gazelle".to_string(), Some(vec!["Hubschrauber".to_string(), "Kalter Krieg".to_string()])));

  return models;

}

//TODO: To Struct

#[allow(unused)]
pub fn get_props() -> Vec<RoleOption> {

  let mut models: Vec<RoleOption> = Vec::new();
  models.push(build_role_option("bf109".to_string(), "Pilot Bf 109 K-4 Kurfürst".to_string(), Some(vec!["WWII".to_string()])));
  models.push(build_role_option("ce2".to_string(), "Pilot Christen Eagle II".to_string(), Some(vec!["Modern".to_string()])));
  models.push(build_role_option("fw190a8".to_string(), "Pilot Fw 190 A-8 Anton".to_string(), Some(vec!["WWII".to_string()])));
  models.push(build_role_option("fw190d9".to_string(), "Pilot Fw 190 D-9 Dora".to_string(), Some(vec!["WWII".to_string()])));
  models.push(build_role_option("i16".to_string(), "Pilot I-16".to_string(), Some(vec!["WWII".to_string()])));
  models.push(build_role_option("mosq".to_string(), "Pilot Mosquito FB VI".to_string(), Some(vec!["WWII".to_string()])));
  models.push(build_role_option("p47".to_string(), "Pilot P-47D Thunderbolt".to_string(), Some(vec!["WWII".to_string()])));
  models.push(build_role_option("p51".to_string(), "Pilot P-51D Mustang".to_string(), Some(vec!["WWII".to_string()])));
  models.push(build_role_option("spitfire".to_string(), "Pilot Spitfire LF Mk. IX".to_string(), Some(vec!["WWII".to_string()])));
  models.push(build_role_option("tf51".to_string(), "Pilot TF-51D".to_string(), Some(vec!["WWII".to_string()])));
  models.push(build_role_option("yak52".to_string(), "Pilot Yak-52".to_string(), Some(vec!["Kalter Krieg".to_string()])));

  return models;

}

#[allow(unused)]
pub fn get_other() -> Vec<RoleOption> {

  let mut roles: Vec<RoleOption> = Vec::new();
  roles.push(build_role_option("BuddySpike".to_string(), "BuddySpike".to_string(), Some(vec!["BuddySpike".to_string()])));
  roles.push(build_role_option("Cockpitbauer".to_string(), "Cockpitbauer".to_string(), Some(vec!["Cockpitbauer".to_string()])));
  roles.push(build_role_option("Liberation".to_string(), "Liberation".to_string(), Some(vec!["Liberation".to_string()])));
  roles.push(build_role_option("PVP".to_string(), "PVP".to_string(), Some(vec!["PVP".to_string()])));
  roles.push(build_role_option("Rotorheads".to_string(), "Rotorheads".to_string(), Some(vec!["Rotorheads".to_string()])));
  roles.push(build_role_option("RotorOps".to_string(), "RotorOps".to_string(), Some(vec!["RotorOps".to_string()])));
  roles.push(build_role_option("VirtualReality".to_string(), "Virtual Reality".to_string(), Some(vec!["Virtual Reality".to_string()])));



  return roles;
}

#[allow(unused)]
pub fn get_all() -> Vec<RoleOption> {

  let mut models: Vec<RoleOption> = Vec::new();
  models.extend(get_jets());
  models.extend(get_helis());
  models.extend(get_props());
  models.extend(get_fc());

  return models;

}
