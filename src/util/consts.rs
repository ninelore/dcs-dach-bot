use std::{collections::HashMap};

pub fn get_models() -> HashMap<String, Vec<String>> {

  let mut models: HashMap<String, Vec<String>> = HashMap::new();
  models.insert("A-10A".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("A-10C Warthog".to_string(), vec!["Modern".to_string()]);
  models.insert("AH-64D".to_string(), vec!["Hubschrauber".to_string(), "Modern".to_string()]);
  models.insert("AJS-37 Viggen".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("AV-8B Night Attack V/STOL".to_string(), vec!["Modern".to_string()]);
  models.insert("Bf 109 K-4 Kurfürst".to_string(), vec!["WWII".to_string()]);
  models.insert("Black Shark".to_string(), vec!["Hubschrauber".to_string(), "Kalter Krieg".to_string()]);
  models.insert("C-101 Aviojet".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("Christen Eagle II".to_string(), vec!["Modern".to_string()]);
  models.insert("F/A-18C Hornet".to_string(), vec!["Modern".to_string()]);
  models.insert("F-14 Tomcat Pilot".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("F-14 Tomcat RIO".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("F-15C Eagle".to_string(), vec!["Modern".to_string()]);
  models.insert("F-16C Viper".to_string(), vec!["Modern".to_string()]);
  models.insert("F-5E Tiger II".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("F-86F Sabre".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("Fw 190 A-8 Anton".to_string(), vec!["WWII".to_string()]);
  models.insert("Fw 190 D-9 Dora".to_string(), vec!["WWII".to_string()]);
  models.insert("I-16".to_string(), vec!["WWII".to_string()]);
  models.insert("JF-17 Thunder".to_string(), vec!["Modern".to_string()]);
  models.insert("L-39 Albatros".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("M-2000C".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("Mi-24P Hind".to_string(), vec!["Hubschrauber".to_string(), "Kalter Krieg".to_string()]);
  models.insert("Mi-8MTV2 Magnificent Eight".to_string(), vec!["Hubschrauber".to_string(), "Kalter Krieg".to_string()]);
  models.insert("MiG-15bis".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("MiG-19P Farmer".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("MiG-21bis".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("MiG-29".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("Mirage F1".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("Mosquito FB VI".to_string(), vec!["WWII".to_string()]);
  models.insert("P-47D Thunderbolt".to_string(), vec!["WWII".to_string()]);
  models.insert("P-51D Mustang".to_string(), vec!["WWII".to_string()]);
  models.insert("SA342 Gazelle".to_string(), vec!["Hubschrauber".to_string(), "Kalter Krieg".to_string()]);
  models.insert("Spitfire LF Mk. IX".to_string(), vec!["WWII".to_string()]);
  models.insert("Su-25A".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("Su-25T".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("Su-27".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("Su-33".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("TF-51D".to_string(), vec!["Kalter Krieg".to_string()]);
  models.insert("UH-1H Huey".to_string(), vec!["Hubschrauber".to_string(), "Kalter Krieg".to_string()]);
  models.insert("Yak-52".to_string(), vec!["Kalter Krieg".to_string()]);

  return models;

}
