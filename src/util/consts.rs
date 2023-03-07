use indexmap::IndexMap;

#[allow(unused)]
pub fn get_jets() -> IndexMap<String, Vec<String>> {

  let mut models: IndexMap<String, Vec<String>> = IndexMap::new();
  models.insert("a10c".to_string(), vec!["A-10C Warthog".to_string(), "Modern".to_string()]);
  models.insert("ajs37".to_string(), vec!["AJS-37 Viggen".to_string(), "Kalter Krieg".to_string()]);
  models.insert("av8b".to_string(), vec!["AV-8B Night Attack V/STOL".to_string(), "Modern".to_string()]);
  models.insert("c101".to_string(), vec!["C-101 Aviojet".to_string(), "Kalter Krieg".to_string()]);
  models.insert("f18c".to_string(), vec!["F/A-18C Hornet".to_string(), "Modern".to_string()]);
  models.insert("f14p".to_string(), vec!["F-14 Tomcat Pilot".to_string(), "Kalter Krieg".to_string()]);
  models.insert("f14r".to_string(), vec!["F-14 Tomcat RIO".to_string(), "Kalter Krieg".to_string()]);
  models.insert("f16c".to_string(), vec!["F-16C Viper".to_string(), "Modern".to_string()]);
  models.insert("f5".to_string(), vec!["F-5E Tiger II".to_string(), "Kalter Krieg".to_string()]);
  models.insert("f86".to_string(), vec!["F-86F Sabre".to_string(), "Kalter Krieg".to_string()]);
  models.insert("jf17".to_string(), vec!["JF-17 Thunder".to_string(), "Modern".to_string()]);
  models.insert("l39".to_string(), vec!["L-39 Albatros".to_string(), "Kalter Krieg".to_string()]);
  models.insert("m2000".to_string(), vec!["M-2000C".to_string(), "Kalter Krieg".to_string()]);
  models.insert("mig15".to_string(), vec!["MiG-15bis".to_string(), "Kalter Krieg".to_string()]);
  models.insert("mig19".to_string(), vec!["MiG-19P Farmer".to_string(), "Kalter Krieg".to_string()]);
  models.insert("mig21".to_string(), vec!["MiG-21bis".to_string(), "Kalter Krieg".to_string()]);
  models.insert("f1".to_string(), vec!["Mirage F1".to_string(), "Kalter Krieg".to_string()]);

  return models;

}

#[allow(unused)]
pub fn get_fc() -> IndexMap<String, Vec<String>> {

  let mut models: IndexMap<String, Vec<String>> = IndexMap::new();
  models.insert("a10a".to_string(), vec!["A-10A".to_string(), "Kalter Krieg".to_string()]);
  models.insert("f15c".to_string(), vec!["F-15C Eagle".to_string(), "Modern".to_string()]);
  models.insert("mig29".to_string(), vec!["MiG-29".to_string(), "Kalter Krieg".to_string()]);
  models.insert("su25a".to_string(), vec!["Su-25A".to_string(), "Kalter Krieg".to_string()]);
  models.insert("su25t".to_string(), vec!["Su-25T".to_string(), "Kalter Krieg".to_string()]);
  models.insert("su27".to_string(), vec!["Su-27".to_string(), "Kalter Krieg".to_string()]);
  models.insert("su33".to_string(), vec!["Su-33".to_string(), "Kalter Krieg".to_string()]);

  return models;

}

#[allow(unused)]
pub fn get_helis() -> IndexMap<String, Vec<String>> {

  let mut models: IndexMap<String, Vec<String>> = IndexMap::new();
  models.insert("ah64d".to_string(), vec!["AH-64D".to_string(), "Hubschrauber".to_string(), "Modern".to_string()]);
  models.insert("ka50".to_string(), vec!["Black Shark".to_string(), "Hubschrauber".to_string(), "Kalter Krieg".to_string()]);
  models.insert("mi24".to_string(), vec!["Mi-24P Hind".to_string(), "Hubschrauber".to_string(), "Kalter Krieg".to_string()]);
  models.insert("mi8".to_string(), vec!["Mi-8MTV2 Magnificent Eight".to_string(), "Hubschrauber".to_string(), "Kalter Krieg".to_string()]);
  models.insert("uh1h".to_string(), vec!["UH-1H Huey".to_string(), "Hubschrauber".to_string(), "Kalter Krieg".to_string()]);
  models.insert("sa342".to_string(), vec!["SA342 Gazelle".to_string(), "Hubschrauber".to_string(), "Kalter Krieg".to_string()]);

  return models;

}

#[allow(unused)]
pub fn get_props() -> IndexMap<String, Vec<String>> {

  let mut models: IndexMap<String, Vec<String>> = IndexMap::new();
  models.insert("bf109".to_string(), vec!["Bf 109 K-4 Kurfürst".to_string(), "WWII".to_string()]);
  models.insert("ce2".to_string(), vec!["Christen Eagle II".to_string(), "Modern".to_string()]);
  models.insert("fw190a8".to_string(), vec!["Fw 190 A-8 Anton".to_string(), "WWII".to_string()]);
  models.insert("fw190d9".to_string(), vec!["Fw 190 D-9 Dora".to_string(), "WWII".to_string()]);
  models.insert("i16".to_string(), vec!["I-16".to_string(), "WWII".to_string()]);
  models.insert("mosq".to_string(), vec!["Mosquito FB VI".to_string(), "WWII".to_string()]);
  models.insert("p47".to_string(), vec!["P-47D Thunderbolt".to_string(), "WWII".to_string()]);
  models.insert("p51".to_string(), vec!["P-51D Mustang".to_string(), "WWII".to_string()]);
  models.insert("spitfire".to_string(), vec!["Spitfire LF Mk. IX".to_string(), "WWII".to_string()]);
  models.insert("tf51".to_string(), vec!["TF-51D".to_string(), "Kalter Krieg".to_string()]);
  models.insert("yak52".to_string(), vec!["Yak-52".to_string(), "Kalter Krieg".to_string()]);

  return models;

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