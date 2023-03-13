use indexmap::IndexMap;

pub struct RoleOption {
  id: String,
  name: String,
  add_role_names: Vec<String>
}

fn build_role_option(id: String, name: String, add_roles: Vec<String>) -> RoleOption {
  RoleOption { id: id, name: name, add_role_names: add_roles }
}

#[allow(unused)]
pub fn get_jets() -> Vec<RoleOption> {

  let models: Vec<RoleOption> = Vec::new();
  models.push(build_role_option("a10c".to_string(), "Pilot A-10C Warthog".to_string(), vec!["Modern".to_string()]));
  models.push(build_role_option("ajs37".to_string(), "Pilot AJS-37 Viggen".to_string(), vec!["Kalter Krieg".to_string()]));
  models.push(build_role_option("av8b".to_string(), "Pilot AV-8B Night Attack V/STOL".to_string(), vec!["Modern".to_string()]));
  models.push(build_role_option("c101".to_string(), "Pilot C-101 Aviojet".to_string(), vec!["Modern".to_string()]));
  models.push(build_role_option("f18c".to_string(), "Pilot F/A-18C Hornet".to_string(), vec!["Modern".to_string()]));
  models.push(build_role_option("f14p".to_string(), "Pilot F-14 Tomcat Pilot".to_string(), vec!["Kalter Krieg".to_string()]));
  models.push(build_role_option("f14r".to_string(), "Pilot F-14 Tomcat RIO".to_string(), vec!["Kalter Krieg".to_string()]));
  models.push(build_role_option("f16c".to_string(), "Pilot F-16C Viper".to_string(), vec!["Modern".to_string()]));
  models.push(build_role_option("f5".to_string(), "Pilot F-5E Tiger II".to_string(), vec!["Kalter Krieg".to_string()]));
  models.push(build_role_option("f86".to_string(), "Pilot F-86F Sabre".to_string(), vec!["Kalter Krieg".to_string()]));
  models.push(build_role_option("jf17".to_string(), "Pilot JF-17 Thunder".to_string(), vec!["Modern".to_string()]));
  models.push(build_role_option("l39".to_string(), "Pilot L-39 Albatros".to_string(), vec!["Kalter Krieg".to_string()]));
  models.push(build_role_option("m2000".to_string(), "Pilot M-2000C".to_string(), vec!["Kalter Krieg".to_string()]));
  models.push(build_role_option("mig15".to_string(), "Pilot MiG-15bis".to_string(), vec!["Kalter Krieg".to_string()]));
  models.push(build_role_option("mig19".to_string(), "Pilot MiG-19P Farmer".to_string(), vec!["Kalter Krieg".to_string()]));
  models.push(build_role_option("mig21".to_string(), "Pilot MiG-21bis".to_string(), vec!["Kalter Krieg".to_string()]));
  models.push(build_role_option("f1".to_string(), "Pilot Mirage F1".to_string(), vec!["Kalter Krieg".to_string()]));

  return models;

}

#[allow(unused)]
pub fn get_fc() -> Vec<RoleOption> {

  let models: Vec<RoleOption> = Vec::new();
  models.push(build_role_option("a10a".to_string(), "Pilot A-10A".to_string(), vec!["Kalter Krieg".to_string()]));
  models.push(build_role_option("f15c".to_string(), "Pilot F-15C Eagle".to_string(), vec!["Modern".to_string()]));
  models.push(build_role_option("mig29".to_string(), "Pilot MiG-29".to_string(), vec!["Kalter Krieg".to_string()]));
  models.push(build_role_option("su25a".to_string(), "Pilot Su-25A".to_string(), vec!["Kalter Krieg".to_string()]));
  models.push(build_role_option("su25t".to_string(), "Pilot Su-25T".to_string(), vec!["Kalter Krieg".to_string()]));
  models.push(build_role_option("su27".to_string(), "Pilot Su-27".to_string(), vec!["Kalter Krieg".to_string()]));
  models.push(build_role_option("su33".to_string(), "Pilot Su-33".to_string(), vec!["Kalter Krieg".to_string()]));

  return models;

}

#[allow(unused)]
pub fn get_helis() -> Vec<RoleOption> {

  let mut models: Vec<RoleOption> = Vec::new();
  models.push(build_role_option("ah64d".to_string(), "Pilot AH-64D".to_string(), vec!["Hubschrauber".to_string(), "Modern".to_string()]));
  models.push(build_role_option("ka50".to_string(), "Pilot Black Shark".to_string(), vec!["Hubschrauber".to_string(), "Kalter Krieg".to_string()]));
  models.push(build_role_option("mi24".to_string(), "Pilot Mi-24P Hind".to_string(), vec!["Hubschrauber".to_string(), "Kalter Krieg".to_string()]));
  models.push(build_role_option("mi8".to_string(), "Pilot Mi-8MTV2 Magnificent Eight".to_string(), vec!["Hubschrauber".to_string(), "Kalter Krieg".to_string()]));
  models.push(build_role_option("uh1h".to_string(), "Pilot UH-1H Huey".to_string(), vec!["Hubschrauber".to_string(), "Kalter Krieg".to_string()]));
  models.push(build_role_option("sa342".to_string(), "Pilot SA342 Gazelle".to_string(), vec!["Hubschrauber".to_string(), "Kalter Krieg".to_string()]));

  return models;

}

//TODO: To Struct

#[allow(unused)]
pub fn get_props() -> IndexMap<String, Vec<String>> {

  let mut models: IndexMap<String, Vec<String>> = IndexMap::new();
  models.insert("bf109".to_string(), vec!["Pilot Bf 109 K-4 Kurfürst".to_string(), "WWII".to_string()]);
  models.insert("ce2".to_string(), vec!["Pilot Christen Eagle II".to_string(), "Modern".to_string()]);
  models.insert("fw190a8".to_string(), vec!["Pilot Fw 190 A-8 Anton".to_string(), "WWII".to_string()]);
  models.insert("fw190d9".to_string(), vec!["Pilot Fw 190 D-9 Dora".to_string(), "WWII".to_string()]);
  models.insert("i16".to_string(), vec!["Pilot I-16".to_string(), "WWII".to_string()]);
  models.insert("mosq".to_string(), vec!["Pilot Mosquito FB VI".to_string(), "WWII".to_string()]);
  models.insert("p47".to_string(), vec!["Pilot P-47D Thunderbolt".to_string(), "WWII".to_string()]);
  models.insert("p51".to_string(), vec!["Pilot P-51D Mustang".to_string(), "WWII".to_string()]);
  models.insert("spitfire".to_string(), vec!["Pilot Spitfire LF Mk. IX".to_string(), "WWII".to_string()]);
  models.insert("tf51".to_string(), vec!["Pilot TF-51D".to_string(), "Kalter Krieg".to_string()]);
  models.insert("yak52".to_string(), vec!["Pilot Yak-52".to_string(), "Kalter Krieg".to_string()]);

  return models;

}

#[allow(unused)]
pub fn get_other() -> IndexMap<String, Vec<String>> {

  let mut roles: IndexMap<String, Vec<String>> = IndexMap::new();
  roles.insert("BuddySpike".to_string(), vec!["BuddySpike".to_string()]);
  roles.insert("Cockpitbauer".to_string(), vec!["Cockpitbauer".to_string()]);
  roles.insert("Liberation".to_string(), vec!["Liberation".to_string()]);
  roles.insert("PVP".to_string(), vec!["PVP".to_string()]);
  roles.insert("Rotorheads".to_string(), vec!["Rotorheads".to_string()]);
  roles.insert("RotorOps".to_string(), vec!["RotorOps".to_string()]);
  roles.insert("VirtualReality".to_string(), vec!["Virtual Reality".to_string()]);



  return roles;
}

#[allow(unused)]
pub fn get_all() -> IndexMap<String, Vec<String>> {

  let mut models: IndexMap<String, Vec<String>> = IndexMap::new();
  models.extend(get_jets());
  models.extend(get_helis());
  models.extend(get_props());
  models.extend(get_fc());

  return models;

}
