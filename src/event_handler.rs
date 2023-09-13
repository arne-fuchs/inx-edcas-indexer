use std::process;
use std::str::FromStr;
use std::sync::Arc;
use chrono::{DateTime, Duration, Utc};
use json::JsonValue;
use log::warn;
use tokio::sync::Mutex;

pub async fn handle_event(json: JsonValue, client: Arc<Mutex<tokio_postgres::Client>>) {
    let timestamp = Utc::now().timestamp();
    let mut message = json.clone();
    if !json["message"].is_null() {
        message = json["message"].clone();
    }

    //Check if data is too old (false data)
    let parsed_date_time = DateTime::parse_from_rfc3339(message["timestamp"].as_str().unwrap()).unwrap();
    let current_date_time = Utc::now();
    let max_age = Duration::hours(1);
    let time_difference = current_date_time.signed_duration_since(parsed_date_time);
    if time_difference > max_age {
        println!("Found too old data(Current: {} Found: {}): {}",current_date_time, parsed_date_time, json);
        return;
    }

    let event_result = message["event"].as_str();
    let mut event = "None";
    match event_result {
        None => {}
        Some(result) => { event = result; }
    }

    if client.lock().await.is_closed(){
        process::exit(20);
    }

    match event {
        //Navigation
        //{ "timestamp":"2022-10-16T20:54:45Z", "event":"Location", "DistFromStarLS":1007.705243, "Docked":true, "StationName":"Q2K-BHB", "StationType":"FleetCarrier", "MarketID":3704402432, "StationFaction":{ "Name":"FleetCarrier" }, "StationGovernment":"$government_Carrier;", "StationGovernment_Localised":"Privateigentum", "StationServices":[ "dock", "autodock", "commodities", "contacts", "exploration", "outfitting", "crewlounge", "rearm", "refuel", "repair", "shipyard", "engineer", "flightcontroller", "stationoperations", "stationMenu", "carriermanagement", "carrierfuel", "livery", "voucherredemption", "socialspace", "bartender", "vistagenomics" ], "StationEconomy":"$economy_Carrier;", "StationEconomy_Localised":"Privatunternehmen", "StationEconomies":[ { "Name":"$economy_Carrier;", "Name_Localised":"Privatunternehmen", "Proportion":1.000000 } ], "Taxi":false, "Multicrew":false, "StarSystem":"Colonia", "SystemAddress":3238296097059, "StarPos":[-9530.50000,-910.28125,19808.12500], "SystemAllegiance":"Independent", "SystemEconomy":"$economy_Tourism;", "SystemEconomy_Localised":"Tourismus", "SystemSecondEconomy":"$economy_HighTech;", "SystemSecondEconomy_Localised":"Hightech", "SystemGovernment":"$government_Cooperative;", "SystemGovernment_Localised":"Kooperative", "SystemSecurity":"$SYSTEM_SECURITY_low;", "SystemSecurity_Localised":"Geringe Sicherheit", "Population":583869, "Body":"Colonia 2 c", "BodyID":18, "BodyType":"Planet", "Factions":[ { "Name":"Jaques", "FactionState":"Investment", "Government":"Cooperative", "Influence":0.454092, "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand1;", "Happiness_Localised":"In Hochstimmung", "MyReputation":100.000000, "RecoveringStates":[ { "State":"PublicHoliday", "Trend":0 } ], "ActiveStates":[ { "State":"Investment" }, { "State":"CivilLiberty" } ] }, { "Name":"Colonia Council", "FactionState":"Boom", "Government":"Cooperative", "Influence":0.331337, "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Glücklich", "MyReputation":100.000000, "ActiveStates":[ { "State":"Boom" } ] }, { "Name":"People of Colonia", "FactionState":"None", "Government":"Cooperative", "Influence":0.090818, "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Glücklich", "MyReputation":27.956400 }, { "Name":"Holloway Bioscience Institute", "FactionState":"None", "Government":"Corporate", "Influence":0.123752, "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Glücklich", "MyReputation":-9.420000, "RecoveringStates":[ { "State":"PirateAttack", "Trend":0 } ] } ], "SystemFaction":{ "Name":"Jaques", "FactionState":"Investment" } }
        "FSDJump" | "Location" | "CarrierJump" => {
            //{ "timestamp":"2022-10-16T23:25:31Z", "event":"FSDJump", "Taxi":false, "Multicrew":false, "StarSystem":"Ogmar",
            // "SystemAddress":84180519395914, "StarPos":[-9534.00000,-905.28125,19802.03125], "SystemAllegiance":"Independent",
            // "SystemEconomy":"$economy_HighTech;", "SystemEconomy_Localised":"Hightech", "SystemSecondEconomy":"$economy_Military;",
            // "SystemSecondEconomy_Localised":"Militär", "SystemGovernment":"$government_Confederacy;", "SystemGovernment_Localised":"Konföderation",
            // "SystemSecurity":"$SYSTEM_SECURITY_medium;", "SystemSecurity_Localised":"Mittlere Sicherheit", "Population":151752, "Body":"Ogmar A",
            // "BodyID":1, "BodyType":"Star", "JumpDist":8.625, "FuelUsed":0.024493, "FuelLevel":31.975506,

            // "Factions":[
            // { "Name":"Jaques", "FactionState":"Election", "Government":"Cooperative", "Influence":0.138384, "Allegiance":"Independent",
            // "Happiness":"$Faction_HappinessBand1;", "Happiness_Localised":"In Hochstimmung", "MyReputation":100.000000,
            // "PendingStates":[ { "State":"Outbreak", "Trend":0 } ], "ActiveStates":[ { "State":"Election" } ] },

            // { "Name":"ICU Colonial Corps", "FactionState":"War", "Government":"Communism", "Influence":0.119192, "Allegiance":"Independent",
            // "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Glücklich", "MyReputation":96.402496,
            // "PendingStates":[ { "State":"Expansion", "Trend":0 } ], "ActiveStates":[ { "State":"War" } ] },

            // { "Name":"Societas Eruditorum de Civitas Dei", "FactionState":"War", "Government":"Dictatorship",
            // "Influence":0.119192, "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Glücklich",
            // "MyReputation":46.414799, "ActiveStates":[ { "State":"War" } ] },

            // { "Name":"GalCop Colonial Defence Commission",
            // "FactionState":"Boom", "Government":"Confederacy", "Influence":0.406061, "Allegiance":"Independent",
            // "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Glücklich", "MyReputation":-75.000000,
            // "ActiveStates":[ { "State":"Boom" } ] },

            // { "Name":"Likedeeler of Colonia", "FactionState":"None",
            // "Government":"Democracy", "Influence":0.068687, "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand2;",
            // "Happiness_Localised":"Glücklich", "MyReputation":4.002500 },

            // { "Name":"Colonia Tech Combine", "FactionState":"Election",
            // "Government":"Cooperative", "Influence":0.138384, "Allegiance":"Independent", "Happiness":"$Faction_HappinessBand2;",
            // "Happiness_Localised":"Glücklich", "MyReputation":4.850000, "ActiveStates":[ { "State":"Election" } ] },

            // "SystemFaction":{ "Name":"GalCop Colonial Defence Commission", "FactionState":"Boom" },
            // "Conflicts":[ { "WarType":"election", "Status":"active", "Faction1":{ "Name":"Jaques", "Stake":"Guerrero Military Base", "WonDays":1 },
            // "Faction2":{ "Name":"Colonia Tech Combine", "Stake":"", "WonDays":0 } }, { "WarType":"war", "Status":"active",
            // "Faction1":{ "Name":"ICU Colonial Corps", "Stake":"Boulaid Command Facility", "WonDays":1 },
            // "Faction2":{ "Name":"Societas Eruditorum de Civitas Dei", "Stake":"Chatterjee's Respite", "WonDays":0 } } ] }


            //{"Body":"BD+15 1957","BodyID":1,"BodyType":"Star","Multicrew":false,"Population":0,"StarPos":[23.28125,30.65625,-35.09375],"StarSystem":"BD+15 1957","SystemAddress":5031721997002,"SystemAllegiance":"","SystemEconomy":"$economy_None;","SystemGovernment":"$government_None;","SystemSecondEconomy":"$economy_None;","SystemSecurity":"$GAlAXY_MAP_INFO_state_anarchy;","Taxi":false,"event":"FSDJump","horizons":true,"odyssey":true,"timestamp":"2023-06-25T21:08:01Z"}

            {
                let name = message["StarSystem"].to_string();
                let address = message["SystemAddress"].as_i64().unwrap();
                let population = message["Population"].as_i32();
                let allegiance = message["SystemAllegiance"].to_string();
                let economy = message["SystemEconomy"].to_string();
                let second_economy = message["SystemSecondEconomy"].to_string();
                let government = message["SystemGovernment"].to_string();
                let security = message["SystemSecurity"].to_string();
                let faction = message["SystemFaction"]["Name"].to_string();
                //"StarPos":[-9534.00000,-905.28125,19802.03125],
                let mut star_pos = message["StarPos"].to_string();
                star_pos = star_pos.replace("[", "");
                star_pos = star_pos.replace("]", "");
                let mut string_split = star_pos.split(",");

                let x: f32 = f32::from_str(string_split.next().unwrap()).unwrap();
                let y: f32 = f32::from_str(string_split.next().unwrap()).unwrap();
                let z: f32 = f32::from_str(string_split.next().unwrap()).unwrap();

                let odyssey = message["odyssey"].as_bool().unwrap_or(true);

                //language=postgresql
                let insert = "
                    INSERT INTO system
                        (timestamp, name, address, population, allegiance, economy, second_economy, government, security, faction, x, y, z, odyssey)
                    VALUES
                        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                    ON CONFLICT (address,odyssey) DO UPDATE SET
                        timestamp = excluded.timestamp,
                        name = excluded.name,
                        address = excluded.address,
                        population = excluded.population,
                        allegiance = excluded.allegiance,
                        economy = excluded.economy,
                        second_economy = excluded.second_economy,
                        government = excluded.government,
                        security = excluded.security,
                        faction = excluded.faction,
                        x = excluded.x,
                        y = excluded.y,
                        z = excluded.z,
                        odyssey = excluded.odyssey;";
                client.lock().await.execute(insert,
                                            &[&timestamp, &name, &address, &population, &allegiance, &economy, &second_economy, &government, &security, &faction, &x, &y, &z, &odyssey],
                ).await.unwrap();

                //language=postgresql
                let delete = "DELETE FROM system_faction WHERE system_address = $1 and odyssey = $2;";
                client.lock().await.execute(delete, &[&address, &odyssey]).await.unwrap();
                //TODO  faction_active_state, faction_recovering_state, conflicts

                for i in 0..message["Factions"].len() {
                    let name = message["Factions"][i]["Name"].to_string();
                    let address = message["SystemAddress"].as_i64().unwrap();
                    let faction_state = message["Factions"][i]["FactionState"].to_string();
                    let government = message["Factions"][i]["Government"].to_string();
                    let influence = message["Factions"][i]["Influence"].as_f32().unwrap();
                    let allegiance = message["Factions"][i]["Allegiance"].to_string();
                    let happiness = message["Factions"][i]["Happiness"].to_string();

                    // "Factions":[
                    // { "Name":"Milanov's Reavers", "FactionState":"Bust", "Government":"Anarchy", "Influence":0.010101, "Allegiance":"Independent",
                    // "Happiness":"$Faction_HappinessBand2;", "Happiness_Localised":"Glücklich", "MyReputation":0.000000,
                    // "RecoveringStates":[ { "State":"Terrorism", "Trend":0 } ], "ActiveStates":[ { "State":"Bust" } ] } ],

                    //language=postgresql
                    let insert = "INSERT INTO system_faction (timestamp, name, system_address, faction_state, government, influence, allegiance, happiness, odyssey) VALUES ($1,$2,$3,$4,$5,$6,$7,$8, $9) ON CONFLICT (system_address,name,odyssey) DO UPDATE SET
                                                                                                                                                                                                                                timestamp = excluded.timestamp,
                                                                                                                                                                                                                                name = excluded.name,
                                                                                                                                                                                                                                system_address = excluded.system_address,
                                                                                                                                                                                                                                faction_state = excluded.faction_state,
                                                                                                                                                                                                                                government = excluded.government,
                                                                                                                                                                                                                                influence = excluded.influence,
                                                                                                                                                                                                                                allegiance = excluded.allegiance,
                                                                                                                                                                                                                                happiness = excluded.happiness,
                                                                                                                                                                                                                                odyssey = excluded.odyssey;";
                    match client.lock().await.execute(insert,
                                                      &[&timestamp, &name, &address, &faction_state, &government, &influence, &allegiance, &happiness, &odyssey],
                    ).await {
                        Ok(_) => {}
                        Err(err) => {
                            if !err.to_string().contains("violates foreign key constraint") || !err.to_string().contains("duplicate key value violates unique constraint") {
                                panic!("{}", err);
                            }
                        }
                    }

                    //TODO  faction_active_state, faction_recovering_state, conflicts
                }
            }
        }
        "SupercruiseEntry" => {}
        "SupercruiseExit" => {}
        //{ "timestamp":"2022-10-16T23:25:05Z", "event":"StartJump", "JumpType":"Hyperspace", "StarSystem":"Ogmar", "SystemAddress":84180519395914, "StarClass":"K" }
        "StartJump" => {} //If jump has been initialised
        //{ "timestamp":"2022-10-16T23:24:46Z", "event":"FSDTarget", "Name":"Ogmar", "SystemAddress":84180519395914, "StarClass":"K", "RemainingJumpsInRoute":1 }
        "FSDTarget" => {} //If system has been targeted
        "NavRoute" => {} //If route has been set -> check json for further information
        "NavRouteClear" => {} //If navigation is complete -> no further information

        //Approaching
        "ApproachSettlement" => {}
        "ApproachBody" => {}
        "LeaveBody" => {}
        "Liftoff" => {}
        "Touchdown" => {}
        "Embark" => {}
        "Disembark" => {}

        //Scanning
        "DiscoveryScan" => {}
        "FSSAllBodiesFound" => {}
        //{ "timestamp":"2022-10-16T23:46:48Z", "event":"FSSDiscoveryScan", "Progress":0.680273, "BodyCount":21, "NonBodyCount":80, "SystemName":"Ogmar", "SystemAddress":84180519395914 }
        "FSSDiscoveryScan" => {}//Honk
        //{ "timestamp":"2022-07-07T20:58:06Z", "event":"SAASignalsFound", "BodyName":"IC 2391 Sector YE-A d103 B 1", "SystemAddress":3549631072611, "BodyID":15, "Signals":[ { "Type":"$SAA_SignalType_Guardian;", "Type_Localised":"Guardian", "Count":1 }, { "Type":"$SAA_SignalType_Human;", "Type_Localised":"Menschlich", "Count":9 } ] }
        "FSSBodySignals" | "SAASignalsFound" => {}
        "FSSSignalDiscovered" => {
            //{ "timestamp":"2023-05-29T22:40:26Z", "event":"FSSSignalDiscovered", "SystemAddress":672296347049, "SignalName":"$MULTIPLAYER_SCENARIO80_TITLE;", "SignalName_Localised":"Unbewachtes Navigationssignal" }
            // { "timestamp":"2023-05-29T22:40:26Z", "event":"FSSSignalDiscovered", "SystemAddress":672296347049, "SignalName":"THE GENERAL MELCHETT X5W-0XL", "IsStation":true }
            //{ "timestamp":"2023-05-29T22:40:42Z", "event":"FSSSignalDiscovered", "SystemAddress":672296347049, "SignalName":"$USS_HighGradeEmissions;", "SignalName_Localised":"Unidentifizierte Signalquelle",
            // "USSType":"$USS_Type_ValuableSalvage;", "USSType_Localised":"Verschlüsselte Emissionen", "SpawningState":"", "SpawningFaction":"Murus Major Industry", "ThreatLevel":0, "TimeRemaining":707.545837 }
        }
        "SAAScanComplete" => {}
        "Scan" => {
            //TODO Rings
            //TODO Cluster
            //TODO atmosphere_composition
            //TODO composition

            let system_address = message["SystemAddress"].as_i64().unwrap();
            let id = message["BodyID"].as_i32().unwrap();
            let name = message["BodyName"].to_string();

            let ascending_node = message["AscendingNode"].as_f32();
            let axial_tilt = message["AxialTilt"].as_f32();
            let atmosphere = message["Atmosphere"].to_string();
            let distance_from_arrival_ls = message["DistanceFromArrivalLS"].as_f32().unwrap();
            let eccentricity = message["Eccentricity"].as_f32();
            let landable = message["Landable"].as_bool().unwrap_or(false);
            let mass_em = message["MassEM"].as_f32();
            let mean_anomaly = message["MeanAnomaly"].as_f32();
            let orbital_inclination = message["OrbitalInclination"].as_f32();
            let orbital_period = message["OrbitalPeriod"].as_f32();
            let periapsis = message["Periapsis"].as_f32();
            let class = message["PlanetClass"].to_string();
            let radius = message["Radius"].as_f32();
            let rotation_period = message["RotationPeriod"].as_f32();
            let semi_major_axis = message["SemiMajorAxis"].as_f32();
            let surface_gravity = message["SurfaceGravity"].as_f32();
            let surface_pressure = message["SurfacePressure"].as_f32();
            let surface_temperature = message["SurfaceTemperature"].as_f32();
            let terraform_state = message["TerraformState"].to_string();
            let tidal_lock = message["TidalLock"].as_bool();
            let volcanism = message["Volcanism"].to_string();
            let discovered = message["WasDiscovered"].as_bool().unwrap();
            let mapped = message["WasMapped"].as_bool().unwrap();

            let odyssey = message["odyssey"].as_bool().unwrap_or(true);

            if message["StarType"].is_null() {
                //Body
                //language=postgresql
                let sql = "
                    INSERT INTO body (timestamp, system_address, id, name, ascending_node, axial_tilt, atmosphere, distance_from_arrival_ls,
                    eccentricity, landable, mass_em, mean_anomaly, orbital_inclination, orbital_period, periapsis, class,
                    radius, rotation_period, semi_major_axis, surface_gravity,
                    surface_pressure, surface_temperature, terraform_state, tidal_lock, volcanism, discovered, mapped,odyssey)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28)
                    ON CONFLICT (system_address,id,odyssey) DO UPDATE SET
                          timestamp                = excluded.timestamp,
                          system_address              = excluded.system_address,
                          id                       = excluded.id,
                          name                     = excluded.name,
                          ascending_node           = excluded.ascending_node,
                          axial_tilt               = excluded.axial_tilt,
                          atmosphere               = excluded.atmosphere,
                          distance_from_arrival_ls = excluded.distance_from_arrival_ls,
                          eccentricity             = excluded.eccentricity,
                          landable                 = excluded.landable,
                          mass_em                  = excluded.mass_em,
                          mean_anomaly             = excluded.mean_anomaly,
                          orbital_inclination      = excluded.orbital_inclination,
                          orbital_period           = excluded.orbital_period,
                          periapsis                = excluded.periapsis,
                          class                    = excluded.class,
                          radius                   = excluded.radius,
                          rotation_period          = excluded.rotation_period,
                          semi_major_axis          = excluded.semi_major_axis,
                          surface_gravity          = excluded.surface_gravity,
                          surface_pressure         = excluded.surface_pressure,
                          surface_temperature      = excluded.surface_temperature,
                          terraform_state          = excluded.terraform_state,
                          tidal_lock               = excluded.tidal_lock,
                          volcanism                = excluded.volcanism,
                          discovered               = excluded.discovered,
                          mapped                   = excluded.mapped,
                          odyssey                  = excluded.odyssey;";
                match client.lock().await.execute(sql, &[
                    &timestamp, &system_address, &id, &name, &ascending_node, &axial_tilt, &atmosphere, &distance_from_arrival_ls,
                    &eccentricity, &landable, &mass_em, &mean_anomaly, &orbital_inclination, &orbital_period, &periapsis, &class, &radius,
                    &rotation_period, &semi_major_axis, &surface_gravity, &surface_pressure, &surface_temperature, &terraform_state, &tidal_lock,
                    &volcanism, &discovered, &mapped, &odyssey
                ]).await {
                    Ok(_) => {}
                    Err(err) => {
                        if !err.to_string().contains("violates foreign key constraint") {
                            panic!("{}", err);
                        }
                    }
                }
                {
                    //Body composition
                    // "Composition":{"Ice":0,"Metal":0.327036,"Rock":0.672964},

                    let mut composition_entries = message["Composition"].entries();
                    let mut entry_option = composition_entries.next();

                    //Clear existing data if exists
                    if !entry_option.is_none() {
                        //language=postgresql
                        let delete = "DELETE FROM body_composition WHERE body_id=$1 and system_address=$2 and odyssey=$3;";
                        client.lock().await.execute(delete, &[&id, &system_address, &odyssey]).await.unwrap();
                    }

                    while !entry_option.is_none() {
                        let composition = entry_option.unwrap();
                        let name = composition.0;
                        let percentage = composition.1.as_f32().unwrap();
                        //language=postgresql
                        let sql = "INSERT INTO body_composition (timestamp, body_id, system_address, name, percentage, odyssey)
                            VALUES ($1,$2,$3,$4,$5,$6) ON CONFLICT(system_address,odyssey,body_id) DO UPDATE SET
                            timestamp = excluded.timestamp,
                            body_id = excluded.body_id,
                            system_address = excluded.system_address,
                            name = excluded.name,
                            percentage = excluded.percentage,
                            odyssey = excluded.odyssey;";
                        match client.lock().await.execute(sql, &[&timestamp, &id, &system_address, &name, &percentage, &odyssey]).await {
                            Ok(_) => {}
                            Err(err) => {
                                if !err.to_string().contains("violates foreign key constraint") {
                                    panic!("{}", err);
                                }
                            }
                        }

                        entry_option = composition_entries.next();
                    }
                }

                {
                    //Body materials
                    // "Materials":[{"Name":"iron","Percent":21.13699},{"Name":"nickel","Percent":15.987134},{"Name":"sulphur","Percent":15.03525},{"Name":"carbon","Percent":12.643088},{"Name":"chromium","Percent":9.506006},{"Name":"manganese","Percent":8.729361},{"Name":"phosphorus","Percent":8.094321},

                    //Deletes old data if data is present
                    if message["Materials"].len() > 0 {
                        //language=postgresql
                        let delete = "DELETE FROM body_material WHERE body_id=$1 and system_address=$2 and odyssey=$3";
                        client.lock().await.execute(delete, &[&id, &system_address, &odyssey]).await.unwrap();
                    }

                    for i in 0..message["Materials"].len() {
                        let name = message["Materials"][i]["Name"].to_string();
                        let percentage = message["Materials"][i]["Percent"].as_f32().unwrap();
                        //language=postgresql
                        let sql = "INSERT INTO body_material (timestamp, body_id, system_address, name, percentage, odyssey) VALUES ($1,$2,$3,$4,$5,$6) ON CONFLICT (system_address,body_id,name,odyssey) DO UPDATE SET
                                                                                                                                                                                      timestamp = excluded.timestamp,
                                                                                                                                                                                      body_id = excluded.body_id,
                                                                                                                                                                                      system_address = excluded.system_address,
                                                                                                                                                                                      name = excluded.name,
                                                                                                                                                                                      percentage = excluded.percentage,
                                                                                                                                                                                      odyssey = excluded.odyssey;";
                        match client.lock().await.execute(sql, &[&timestamp, &id, &system_address, &name, &percentage, &odyssey]).await {
                            Ok(_) => {}
                            Err(err) => {
                                if !err.to_string().contains("violates foreign key constraint") {
                                    panic!("{}", err);
                                }
                            }
                        }
                    }
                }
                {
                    // "AtmosphereComposition":[ { "Name":"Hydrogen", "Percent":73.044167 }, { "Name":"Helium", "Percent":26.955832 } ],

                    //Clear existing data if exists
                    if !message["AtmosphereComposition"].len() > 0 {
                        //language=postgresql
                        let delete = "DELETE FROM atmosphere_composition WHERE body_id=$1 and system_address=$2 and odyssey=$3;";
                        client.lock().await.execute(delete, &[&id, &system_address, &odyssey]).await.unwrap();
                    }

                    for i in 0..message["AtmosphereComposition"].len() {
                        let name = message["AtmosphereComposition"][i]["Name"].to_string();
                        let percentage = message["AtmosphereComposition"][i]["Percent"].as_f32().unwrap();
                        //language=postgresql
                        let sql = "INSERT INTO atmosphere_composition (timestamp, body_id, system_address, name, percentage, odyssey) VALUES ($1,$2,$3,$4,$5,$6) ON CONFLICT (system_address, body_id, odyssey) DO UPDATE SET
                                                                                                                                                                                      timestamp = excluded.timestamp,
                                                                                                                                                                                      body_id = excluded.body_id,
                                                                                                                                                                                      system_address = excluded.system_address,
                                                                                                                                                                                      name = excluded.name,
                                                                                                                                                                                      percentage = excluded.percentage,
                                                                                                                                                                                      odyssey = excluded.odyssey;";
                        match client.lock().await.execute(sql, &[&timestamp, &id, &system_address, &name, &percentage, &odyssey]).await {
                            Ok(_) => {}
                            Err(err) => {
                                if !err.to_string().contains("violates foreign key constraint") {
                                    panic!("{}", err);
                                }
                            }
                        }
                    }
                }
            } else {
                //Stars only
                let absolute_magnitude = message["AbsoluteMagnitude"].as_f32();
                let age_my = message["Age_MY"].as_i32();
                let luminosity = message["Luminosity"].to_string();
                let star_type = message["StarType"].to_string();
                let stellar_mass = message["StellarMass"].as_f32();
                let subclass = message["Subclass"].as_i32().unwrap();

                //Star
                //language=postgresql
                let sql = "INSERT INTO star (timestamp, system_address, name, id, absolute_magnitude, age_my, ascending_node, axial_tilt,
                    distance_from_arrival_ls, eccentricity, luminosity, mean_anomaly, orbital_inclination, orbital_period,
                    periapsis, radius, rotation_period, semi_major_axis, type, stellar_mass, subclass,
                    surface_temperature, discovered, mapped,odyssey)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25)
                    ON CONFLICT (odyssey,id,system_address) DO UPDATE SET
                          timestamp                = excluded.timestamp,
                          system_address              = excluded.system_address,
                          name                     = excluded.name,
                          id                       = excluded.id,
                          absolute_magnitude       = excluded.absolute_magnitude,
                          age_my                   = excluded.age_my,
                          ascending_node           = excluded.ascending_node,
                          axial_tilt               = excluded.axial_tilt,
                          distance_from_arrival_ls = excluded.distance_from_arrival_ls,
                          eccentricity             = excluded.eccentricity,
                          luminosity               = excluded.luminosity,
                          mean_anomaly             = excluded.mean_anomaly,
                          orbital_inclination      = excluded.orbital_inclination,
                          orbital_period           = excluded.orbital_period,
                          periapsis                = excluded.periapsis,
                          radius                   = excluded.radius,
                          rotation_period          = excluded.rotation_period,
                          semi_major_axis          = excluded.semi_major_axis,
                          type                     = excluded.type,
                          stellar_mass             = excluded.stellar_mass,
                          subclass                 = excluded.subclass,
                          surface_temperature      = excluded.surface_temperature,
                          discovered               = excluded.discovered,
                          mapped                   = excluded.mapped,
                          odyssey                  = excluded.odyssey;";
                match client.lock().await.execute(sql, &[
                    &timestamp, &system_address, &name, &id, &absolute_magnitude, &age_my, &ascending_node, &axial_tilt, &distance_from_arrival_ls, &eccentricity,
                    &luminosity, &mean_anomaly, &orbital_inclination, &orbital_period, &periapsis, &radius, &rotation_period, &semi_major_axis, &star_type, &stellar_mass, &subclass,
                    &surface_temperature, &discovered, &mapped, &odyssey
                ]).await {
                    Ok(_) => {}
                    Err(err) => {
                        if !err.to_string().contains("violates foreign key constraint") {
                            panic!("{}", err);
                        }
                    }
                }
            }

            if !message["Parents"].is_null() {

                for i in 0..message["Parents"].len(){
                    let entry = message["Parents"][i].entries().next().unwrap();
                    //language=postgresql
                    let sql = "INSERT INTO parent(system_address, body_id, parent_type,parent_id,odyssey) VALUES ($1,$2,$3,$4,$5) ON CONFLICT DO NOTHING";
                    match client.lock().await.execute(sql,&[&system_address,&id,&entry.0,&entry.1.as_i32(),&odyssey]).await {
                        Ok(_) => {}
                        Err(err) => {
                            if !err.to_string().contains("violates foreign key constraint") {
                                panic!("{}", err);
                            }
                        }
                    }
                }
            }


            match message["ScanType"].as_str().unwrap() {
                "Detailed" => {
                    //AtmosphereComposition
                    //{ "timestamp":"2022-10-16T23:51:17Z", "event":"Scan", "ScanType":"Detailed", "BodyName":"Ogmar A 6", "BodyID":40, "Parents":[ {"Star":1}, {"Null":0} ],
                    // "StarSystem":"Ogmar", "SystemAddress":84180519395914, "DistanceFromArrivalLS":3376.246435, "TidalLock":false, "TerraformState":"",
                    // "PlanetClass":"Sudarsky class I gas giant", "Atmosphere":"",
                    // "AtmosphereComposition":[ { "Name":"Hydrogen", "Percent":73.044167 }, { "Name":"Helium", "Percent":26.955832 } ],
                    // "Volcanism":"", "MassEM":24.477320, "Radius":22773508.000000, "SurfaceGravity":18.811067, "SurfaceTemperature":62.810730, "SurfacePressure":0.000000,
                    // "Landable":false, "SemiMajorAxis":1304152250289.916992, "Eccentricity":0.252734, "OrbitalInclination":156.334694, "Periapsis":269.403039,
                    // "OrbitalPeriod":990257555.246353, "AscendingNode":-1.479320, "MeanAnomaly":339.074691, "RotationPeriod":37417.276422, "AxialTilt":0.018931,
                    // "WasDiscovered":true, "WasMapped":true }

                    //{"$schemaRef":"https://eddn.edcd.io/schemas/journal/1","header":{"gamebuild":"r294054/r0 ","gameversion":"4.0.0.1502","gatewayTimestamp":"2023-06-15T09:29:27.310819Z","softwareName":"E:D Market Connector [Windows]","softwareVersion":"5.8.1","uploaderID":"6170cfc5352453fd2d6b2ce2d419094645b01fa7"},"message":{
                    // "AscendingNode":110.057068,"Atmosphere":"hot thick carbon dioxide atmosphere","AtmosphereComposition":[{"Name":"CarbonDioxide","Percent":94.605324},
                    // {"Name":"SulphurDioxide","Percent":5.39424}],"AtmosphereType":"CarbonDioxide","AxialTilt":0.103178,"BodyID":5,"BodyName":"Orom A 1",
                    // "Composition":{"Ice":0.009405,"Metal":0.315834,"Rock":0.674761},"DistanceFromArrivalLS":30.691922,"Eccentricity":0.162657,
                    // "Landable":false,"MassEM":0.936153,"MeanAnomaly":350.356055,"OrbitalInclination":4.286039,"OrbitalPeriod":1083833.932877,
                    // "Parents":[{"Star":2},{"Null":1},{"Null":0}],"Periapsis":299.750594,"PlanetClass":"High metal content body",
                    // "Radius":6005185.0,"RotationPeriod":1083838.502077,"ScanType":"AutoScan","SemiMajorAxis":10945867896.080017,
                    // "StarPos":[95.5625,-19.78125,-54.59375],"StarSystem":"Orom","SurfaceGravity":10.346736,"SurfacePressure":14242333.0,
                    // "SurfaceTemperature":1084.132202,"SystemAddress":672296609169,"TerraformState":"","TidalLock":true,"Volcanism":"major rocky magma volcanism",
                    // "WasDiscovered":true,"WasMapped":true,"event":"Scan","horizons":true,"odyssey":true,"timestamp":"2023-06-15T09:29:25Z"}}

                    //----------------------------------------------------------------

                    //stars
                    //{"$schemaRef":"https://eddn.edcd.io/schemas/journal/1","header":{"gamebuild":"r294054/r0 ","gameversion":"4.0.0.1502","gatewayTimestamp":"2023-06-15T09:29:27.638973Z","softwareName":"E:D Market Connector [Windows]","softwareVersion":"5.8.1","uploaderID":"6170cfc5352453fd2d6b2ce2d419094645b01fa7"},
                    // "message":{"AbsoluteMagnitude":9.400711,"Age_MY":3728,"AscendingNode":-156.088452,"AxialTilt":0,"BodyID":2,"BodyName":"Orom A",
                    // "DistanceFromArrivalLS":0,"Eccentricity":0.127359,"Luminosity":"Va","MeanAnomaly":46.742421,"OrbitalInclination":-83.509701,
                    // "OrbitalPeriod":18585659.265518,"Parents":[{"Null":1},{"Null":0}],"Periapsis":96.930698,"Radius":377082912.0,"RotationPeriod":261511.869252,
                    // "ScanType":"AutoScan","SemiMajorAxis":14222702980.041504,"StarPos":[95.5625,-19.78125,-54.59375],"StarSystem":"Orom",
                    // "StarType":"M","StellarMass":0.332031,"Subclass":5,"SurfaceTemperature":2740.0,"SystemAddress":672296609169,"WasDiscovered":true,
                    // "WasMapped":false,"event":"Scan","horizons":true,"odyssey":true,"timestamp":"2023-06-15T09:29:26Z"}}

                    //{"$schemaRef":"https://eddn.edcd.io/schemas/journal/1","header":{"gamebuild":"r294054/r0 ","gameversion":"4.0.0.1502","gatewayTimestamp":"2023-06-15T09:29:28.289996Z","softwareName":"E:D Market Connector [Windows]","softwareVersion":"5.8.1","uploaderID":"6170cfc5352453fd2d6b2ce2d419094645b01fa7"},"message":{
                    // "AbsoluteMagnitude":12.77002,"Age_MY":3728,
                    // "AscendingNode":84.122502,"AxialTilt":0,"BodyID":4,"BodyName":"Orom C","DistanceFromArrivalLS":22818.38935,"Eccentricity":0.209841,"Luminosity":"V",
                    // "MeanAnomaly":286.26959,"OrbitalInclination":91.078681,"OrbitalPeriod":13521534204.483032,"Parents":[{"Null":0}],"Periapsis":193.796424,"Radius":194845264.0,
                    // "Rings":[{"InnerRad":374400000.0,"MassMT":44430000000000.0,"Name":"Orom C A Belt","OuterRad":1254800000.0,"RingClass":"eRingClass_MetalRich"}],
                    // "RotationPeriod":89216.223789,"ScanType":"AutoScan","SemiMajorAxis":5165326356887.817,"StarPos":[95.5625,-19.78125,-54.59375],
                    // "StarSystem":"Orom","StarType":"L","StellarMass":0.140625,"Subclass":3,"SurfaceTemperature":1755.0,"SystemAddress":672296609169,
                    // "WasDiscovered":true,"WasMapped":false,"event":"Scan","horizons":true,"odyssey":true,"timestamp":"2023-06-15T09:29:26Z"}}

                    //{"$schemaRef":"https://eddn.edcd.io/schemas/journal/1","header":{"gamebuild":"r294054/r0 ","gameversion":"4.0.0.1502","gatewayTimestamp":"2023-06-15T09:29:27.964753Z","softwareName":"E:D Market Connector [Windows]","softwareVersion":"5.8.1","uploaderID":"6170cfc5352453fd2d6b2ce2d419094645b01fa7"},"message":{
                    // "AbsoluteMagnitude":15.153152,"Age_MY":3728,"AscendingNode":-156.088452,"AxialTilt":0,"BodyID":3,"BodyName":"Orom B","DistanceFromArrivalLS":239.565577,
                    // "Eccentricity":0.127359,"Luminosity":"V","MeanAnomaly":46.742421,"OrbitalInclination":-83.509701,"OrbitalPeriod":18585659.265518,"Parents":[{"Null":1},{"Null":0}],
                    // "Periapsis":276.930693,"Radius":148871968.0,"RotationPeriod":76033.675205,"ScanType":"AutoScan","SemiMajorAxis":63628426194.19098,"StarPos":[95.5625,-19.78125,-54.59375],
                    // "StarSystem":"Orom","StarType":"T","StellarMass":0.074219,"Subclass":2,"SurfaceTemperature":1160.0,"SystemAddress":672296609169,"WasDiscovered":true,"WasMapped":false,
                    // "event":"Scan","horizons":true,"odyssey":true,"timestamp":"2023-06-15T09:29:26Z"}}
                }
                "AutoScan" => {
                    //atmosphere_composition
                    //{"$schemaRef":"https://eddn.edcd.io/schemas/journal/1","header":{"gamebuild":"r294054/r0 ","gameversion":"4.0.0.1502","gatewayTimestamp":"2023-06-15T09:29:30.495223Z","softwareName":"EDDiscovery","softwareVersion":"16.2.0.0","uploaderID":"4af7eb92e5cec4d0af7b01fc95372f03c028f348"},"message":{"
                    // AscendingNode":146.970846,"Atmosphere":"thin carbon dioxide atmosphere",
                    // "AtmosphereComposition":[{"Name":"CarbonDioxide","Percent":99.009911},{"Name":"SulphurDioxide","Percent":0.990099}],
                    // "AtmosphereType":"CarbonDioxide","AxialTilt":0.210555,"BodyID":2,"BodyName":"Byeia Thaa PG-Y c35 2",
                    // "Composition":{"Ice":0,"Metal":0.327036,"Rock":0.672964},
                    // "DistanceFromArrivalLS":857.717638,"Eccentricity":0.001851,"Landable":true,"MassEM":0.008794,
                    // "Materials":[{"Name":"iron","Percent":21.13699},{"Name":"nickel","Percent":15.987134},{"Name":"sulphur","Percent":15.03525},{"Name":"carbon","Percent":12.643088},{"Name":"chromium","Percent":9.506006},{"Name":"manganese","Percent":8.729361},{"Name":"phosphorus","Percent":8.094321},
                    // {"Name":"zinc","Percent":5.744243},{"Name":"niobium","Percent":1.444601},{"Name":"mercury","Percent":0.923018},{"Name":"technetium","Percent":0.755985}],"MeanAnomaly":339.911808,"OrbitalInclination":-0.157736,"OrbitalPeriod":77082674.503326,"Parents":[{"Star":0}],"Periapsis":262.56146,"PlanetClass":"High metal content body","Radius":1354584.0,"RotationPeriod":103792.263548,
                    // "ScanType":"Detailed","SemiMajorAxis":257584893703.4607,"StarPos":[-2182.3125,-7.375,2820.5],
                    // "StarSystem":"Byeia Thaa PG-Y c35","SurfaceGravity":1.910229,"SurfacePressure":5294.544922,"SurfaceTemperature":183.361938,"SystemAddress":9700955395338,
                    // "TerraformState":"","TidalLock":false,"Volcanism":"","WasDiscovered":false,"WasMapped":false,"event":"Scan","horizons":true,"odyssey":true,"timestamp":"2023-06-15T09:29:27Z"}}

                    //-------------------------------------------------

                    //Stars
                    //{"AbsoluteMagnitude":11.148727,"Age_MY":12811,"AscendingNode":-34.991732,"AxialTilt":0,"BodyID":1,"BodyName":"Col 285 Sector MZ-U b17-4 A",
                    // "DistanceFromArrivalLS":0,"Eccentricity":0.37743,"Luminosity":"Va","MeanAnomaly":89.63359,"OrbitalInclination":2.013284,
                    // "OrbitalPeriod":8588445842.266083,"Parents":[{"Null":0}],"Periapsis":342.18801,"Radius":263232128.0,"RotationPeriod":113187.883591,
                    // "ScanType":"AutoScan","SemiMajorAxis":1439511477947.235,"StarPos":[-99.96875,-14.28125,32.25],"StarSystem":"Col 285 Sector MZ-U b17-4",
                    // "StarType":"M","StellarMass":0.234375,"Subclass":8,"SurfaceTemperature":2193.0,"SystemAddress":9465705276849,"WasDiscovered":true,
                    // "WasMapped":false,"event":"Scan","horizons":true,"odyssey":true,"timestamp":"2023-06-15T13:13:40Z"}

                    //{"AbsoluteMagnitude":13.836273,"Age_MY":5464,"AscendingNode":-77.431252,"AxialTilt":0,"BodyID":4,"BodyName":"Hypiae Aip NK-W b29-7 C",
                    // "DistanceFromArrivalLS":40312.774067,"Eccentricity":0.239537,"Luminosity":"V","MeanAnomaly":324.44499,"OrbitalInclination":-105.247102,
                    // "OrbitalPeriod":45594221949.57733,"Parents":[{"Null":0}],"Periapsis":100.689846,"Radius":185548720.0,"RotationPeriod":98361.732333,"ScanType":
                    // "AutoScan","SemiMajorAxis":10693013072013.855,"StarPos":[4771.21875,66.09375,-5538.71875],"StarSystem":"Hypiae Aip NK-W b29-7","StarType":"L",
                    // "StellarMass":0.117188,"Subclass":8,"SurfaceTemperature":1407.0,"SystemAddress":16128005119233,"WasDiscovered":false,"WasMapped":false,
                    // "event":"Scan","horizons":true,"odyssey":true,"timestamp":"2023-06-15T13:13:27Z"}
                }
                "NavBeaconDetail" => {
                    //{"AscendingNode":59.981854,"Atmosphere":"thin carbon dioxide atmosphere","AtmosphereComposition":[{"Name":"CarbonDioxide","Percent":99.009911},{"Name":"SulphurDioxide","Percent":0.990099}],"AtmosphereType":"CarbonDioxide","AxialTilt":-0.46992,"BodyID":43,"BodyName":"Iota Horologii A 7 d","Composition":{"Ice":0,"Metal":0.089416,"Rock":0.910584},"DistanceFromArrivalLS":2189.493581,"Eccentricity":0.00291,"Landable":true,"MassEM":0.000919,"Materials":[{"Name":"iron","Percent":18.952173},{"Name":"sulphur","Percent":18.461615},{"Name":"carbon","Percent":15.524308},{"Name":"nickel","Percent":14.334629},{"Name":"phosphorus","Percent":9.938927},{"Name":"chromium","Percent":8.523421},{"Name":"germanium","Percent":5.470057},{"Name":"zinc","Percent":5.150491},{"Name":"cadmium","Percent":1.471722},{"Name":"yttrium","Percent":1.131991},{"Name":"tungsten","Percent":1.040665}],"MeanAnomaly":140.729594,"OrbitalInclination":-0.13871,"OrbitalPeriod":428179.824352,"Parents":[{"Planet":37},{"Null":28},{"Star":1},{"Null":0}],"Periapsis":77.930051,"PlanetClass":"Rocky body","Radius":684243.125,"RotationPeriod":428190.902071,"ScanType":"NavBeaconDetail","SemiMajorAxis":444461178.779602,"StarPos":[29.4375,-47.625,-0.9375],"StarSystem":"Iota Horologii","SurfaceGravity":0.782219,"SurfacePressure":1201.276978,"SurfaceTemperature":162.988632,"SystemAddress":422810995051,"TerraformState":"","TidalLock":true,"Volcanism":"","WasDiscovered":true,"WasMapped":true,"event":"Scan","horizons":true,"odyssey":true,"timestamp":"2023-06-26T16:02:15Z"}
                }
                "Basic" => {
                    //{"AscendingNode":-32.015784,"AxialTilt":-0.034478,"BodyID":56,"BodyName":"Zeta Octantis 8","DistanceFromArrivalLS":4813.928719,"Eccentricity":0.189871,"MassEM":9.977272,
                    // "MeanAnomaly":165.198633,"OrbitalInclination":-2.373165,"OrbitalPeriod":43359650.969505,"Parents":[{"Null":50},{"Star":0}],"Periapsis":252.090217,"PlanetClass":"Water world","Radius":12046490.0,"RotationPe>
                }
                _ => {
                    warn!("Unknown scan type : {}", message["ScanType"].as_str().unwrap());
                    warn!("{message}");
                    println!("Unknown scan type : {}", message["ScanType"].as_str().unwrap());
                    println!("{message}");
                }
            }
        }
        //Planet scan with fss
        "ScanBaryCentre" => {}

        //Maintenance
        "RefuelAll" => {}
        "Resupply" => {}
        "Repair" => {}
        "BuyDrones" => {}
        "SellDrones" => {}
        "BuyAmmo" => {}
        //{ "timestamp":"2022-10-16T23:55:55Z", "event":"ReservoirReplenished", "FuelMain":30.905506, "FuelReservoir":1.070000 }
        "ReservoirReplenished" => {}//If reservoir needs to drain more fuel from main tank
        "RepairAll" => {}
        "RebootRepair" => {}
        "RestockVehicle" => {}

        //Docking
        "DockingRequested" => {}
        "DockingGranted" => {}
        "Docked" => {}
        "Undocked" => {}

        //Engineer
        "EngineerProgress" => {}
        "EngineerCraft" => {}
        "EngineerContribution" => {}

        //Ship management
        "Shipyard" => {}
        "StoredShips" => {}
        "ShipyardSwap" => {}
        "ShipLocker" => {}
        "ModuleBuy" => {}
        "Outfitting" => {}
        "ModuleInfo" => {}
        "StoredModules" => {}
        "DockingCancelled" => {}
        "ShipyardBuy" => {}
        "ShipyardNew" => {}
        "ShipyardTransfer" => {}
        "ModuleStore" => {}
        "ModuleSell" => {}
        "ModuleSellRemote" => {}
        "ModuleSwap" => {}

        //On foot
        "Backpack" => {}
        "BackpackChange" => {}
        "CollectItems" => {}
        "UpgradeSuit" => {}
        "Loadout" => {}
        "LoadoutEquipModule" => {}
        "SuitLoadout" => {}
        "UseConsumable" => {}
        "ScanOrganic" => {}

        //Market
        "MarketBuy" => {}
        "Market" => {}
        "MarketSell" => {}

        //SRV
        "LaunchSRV" => {}
        "DockSRV" => {}

        //Ship fight
        "ShipTargeted" => {}
        "UnderAttack" => {}
        "ShieldState" => {}
        "HullDamage" => {}

        //Cargo, Materials & Mining & Drones
        //{ "timestamp":"2022-09-07T20:08:23Z", "event":"Materials",
        // "Raw":[ { "Name":"sulphur", "Name_Localised":"Schwefel", "Count":300 }, { "Name":"manganese", "Name_Localised":"Mangan", "Count":236 }, { "Name":"vanadium", "Count":95 }, { "Name":"nickel", "Count":300 }, { "Name":"phosphorus", "Name_Localised":"Phosphor", "Count":296 }, { "Name":"iron", "Name_Localised":"Eisen", "Count":300 }, { "Name":"germanium", "Count":239 }, { "Name":"chromium", "Name_Localised":"Chrom", "Count":213 }, { "Name":"carbon", "Name_Localised":"Kohlenstoff", "Count":257 }, { "Name":"molybdenum", "Name_Localised":"Molibdän", "Count":153 }, { "Name":"cadmium", "Name_Localised":"Kadmium", "Count":13 }, { "Name":"selenium", "Name_Localised":"Selen", "Count":14 }, { "Name":"mercury", "Name_Localised":"Quecksilber", "Count":19 }, { "Name":"yttrium", "Count":22 }, { "Name":"zinc", "Name_Localised":"Zink", "Count":250 }, { "Name":"ruthenium", "Count":24 }, { "Name":"arsenic", "Name_Localised":"Arsen", "Count":24 }, { "Name":"tungsten", "Name_Localised":"Wolfram", "Count":75 }, { "Name":"tellurium", "Name_Localised":"Tellur", "Count":12 }, { "Name":"tin", "Name_Localised":"Zinn", "Count":131 }, { "Name":"antimony", "Name_Localised":"Antimon", "Count":45 }, { "Name":"niobium", "Name_Localised":"Niob", "Count":44 }, { "Name":"zirconium", "Count":48 }, { "Name":"technetium", "Count":39 }, { "Name":"lead", "Name_Localised":"Blei", "Count":90 }, { "Name":"boron", "Name_Localised":"Bor", "Count":14 }, { "Name":"polonium", "Count":8 } ],
        // "Manufactured":[ { "Name":"hybridcapacitors", "Name_Localised":"Hybridkondensatoren", "Count":197 }, { "Name":"heatdispersionplate", "Name_Localised":"Wärmeverteilungsplatte", "Count":67 }, { "Name":"gridresistors", "Name_Localised":"Gitterwiderstände", "Count":242 }, { "Name":"mechanicalequipment", "Name_Localised":"Mechanisches Equipment", "Count":220 }, { "Name":"fedcorecomposites", "Name_Localised":"Core Dynamics Kompositwerkstoffe", "Count":100 }, { "Name":"protoheatradiators", "Name_Localised":"Proto-Wärmestrahler", "Count":6 }, { "Name":"salvagedalloys", "Name_Localised":"Geborgene Legierungen", "Count":300 }, { "Name":"highdensitycomposites", "Name_Localised":"Komposite hoher Dichte", "Count":200 }, { "Name":"mechanicalscrap", "Name_Localised":"Mechanischer Schrott", "Count":64 }, { "Name":"chemicalprocessors", "Name_Localised":"Chemische Prozessoren", "Count":250 }, { "Name":"focuscrystals", "Name_Localised":"Laserkristalle", "Count":200 }, { "Name":"imperialshielding", "Name_Localised":"Imperiale Schilde", "Count":53 }, { "Name":"precipitatedalloys", "Name_Localised":"Gehärtete Legierungen", "Count":200 }, { "Name":"galvanisingalloys", "Name_Localised":"Galvanisierende Legierungen", "Count":250 }, { "Name":"shieldingsensors", "Name_Localised":"Schildsensoren", "Count":200 }, { "Name":"chemicaldistillery", "Name_Localised":"Chemiedestillerie", "Count":200 }, { "Name":"heatconductionwiring", "Name_Localised":"Wärmeleitungsverdrahtung", "Count":128 }, { "Name":"phasealloys", "Name_Localised":"Phasenlegierungen", "Count":195 }, { "Name":"wornshieldemitters", "Name_Localised":"Gebrauchte Schildemitter", "Count":300 }, { "Name":"shieldemitters", "Name_Localised":"Schildemitter", "Count":250 }, { "Name":"mechanicalcomponents", "Name_Localised":"Mechanische Komponenten", "Count":11 }, { "Name":"compoundshielding", "Name_Localised":"Verbundschilde", "Count":150 }, { "Name":"protolightalloys", "Name_Localised":"Leichte Legierungen (Proto)", "Count":145 }, { "Name":"refinedfocuscrystals", "Name_Localised":"Raffinierte Laserkristalle", "Count":150 }, { "Name":"heatexchangers", "Name_Localised":"Wärmeaustauscher", "Count":6 }, { "Name":"conductiveceramics", "Name_Localised":"Elektrokeramiken", "Count":44 }, { "Name":"uncutfocuscrystals", "Name_Localised":"Fehlerhafte Fokuskristalle", "Count":250 }, { "Name":"temperedalloys", "Name_Localised":"Vergütete Legierungen", "Count":92 }, { "Name":"basicconductors", "Name_Localised":"Einfache Leiter", "Count":140 }, { "Name":"crystalshards", "Name_Localised":"Kristallscherben", "Count":288 }, { "Name":"unknownenergycell", "Name_Localised":"Thargoiden-Energiezelle", "Count":171 }, { "Name":"unknowntechnologycomponents", "Name_Localised":"Technologiekomponenten der Thargoiden", "Count":150 }, { "Name":"unknownenergysource", "Name_Localised":"Sensorenfragment", "Count":100 }, { "Name":"unknowncarapace", "Name_Localised":"Thargoiden-Krustenschale", "Count":220 }, { "Name":"unknownorganiccircuitry", "Name_Localised":"Organischer Schaltkreis der Thargoiden", "Count":100 }, { "Name":"chemicalmanipulators", "Name_Localised":"Chemische Manipulatoren", "Count":72 }, { "Name":"exquisitefocuscrystals", "Name_Localised":"Erlesene Laserkristalle", "Count":89 }, { "Name":"configurablecomponents", "Name_Localised":"Konfigurierbare Komponenten", "Count":36 }, { "Name":"heatvanes", "Name_Localised":"Wärmeleitbleche", "Count":1 }, { "Name":"biotechconductors", "Name_Localised":"Biotech-Leiter", "Count":57 }, { "Name":"conductivepolymers", "Name_Localised":"Leitfähige Polymere", "Count":5 }, { "Name":"thermicalloys", "Name_Localised":"Thermische Legierungen", "Count":150 }, { "Name":"conductivecomponents", "Name_Localised":"Leitfähige Komponenten", "Count":169 }, { "Name":"fedproprietarycomposites", "Name_Localised":"Kompositwerkstoffe", "Count":150 }, { "Name":"electrochemicalarrays", "Name_Localised":"Elektrochemische Detektoren", "Count":133 }, { "Name":"compactcomposites", "Name_Localised":"Kompaktkomposite", "Count":101 }, { "Name":"filamentcomposites", "Name_Localised":"Filament-Komposite", "Count":250 }, { "Name":"chemicalstorageunits", "Name_Localised":"Lagerungseinheiten für Chemiestoffe", "Count":57 }, { "Name":"protoradiolicalloys", "Name_Localised":"Radiologische Legierungen (Proto)", "Count":39 }, { "Name":"guardian_powercell", "Name_Localised":"Guardian-Energiezelle", "Count":300 }, { "Name":"guardian_powerconduit", "Name_Localised":"Guardian-Energieleiter", "Count":250 }, { "Name":"guardian_techcomponent", "Name_Localised":"Guardian-Technologiekomponenten", "Count":160 }, { "Name":"guardian_sentinel_weaponparts", "Name_Localised":"Guardian-Wache-Waffenteile", "Count":200 }, { "Name":"pharmaceuticalisolators", "Name_Localised":"Pharmazeutische Isolatoren", "Count":27 }, { "Name":"militarygradealloys", "Name_Localised":"Militärqualitätslegierungen", "Count":63 }, { "Name":"guardian_sentinel_wreckagecomponents", "Name_Localised":"Guardian-Wrackteilkomponenten", "Count":300 }, { "Name":"heatresistantceramics", "Name_Localised":"Hitzefeste Keramik", "Count":87 }, { "Name":"polymercapacitors", "Name_Localised":"Polymerkondensatoren", "Count":91 }, { "Name":"tg_biomechanicalconduits", "Name_Localised":"Biomechanische Leiter", "Count":105 }, { "Name":"tg_wreckagecomponents", "Name_Localised":"Wrackteilkomponenten", "Count":144 }, { "Name":"tg_weaponparts", "Name_Localised":"Waffenteile", "Count":135 }, { "Name":"tg_propulsionelement", "Name_Localised":"Schubantriebelemente", "Count":100 }, { "Name":"militarysupercapacitors", "Name_Localised":"Militärische Superkondensatoren", "Count":1 }, { "Name":"improvisedcomponents", "Name_Localised":"Behelfskomponenten", "Count":4 } ],
        // "Encoded":[ { "Name":"shielddensityreports", "Name_Localised":"Untypische Schildscans ", "Count":200 }, { "Name":"shieldcyclerecordings", "Name_Localised":"Gestörte Schildzyklus-Aufzeichnungen", "Count":234 }, { "Name":"encryptedfiles", "Name_Localised":"Ungewöhnliche verschlüsselte Files", "Count":92 }, { "Name":"bulkscandata", "Name_Localised":"Anormale Massen-Scan-Daten", "Count":192 }, { "Name":"decodedemissiondata", "Name_Localised":"Entschlüsselte Emissionsdaten", "Count":112 }, { "Name":"encryptioncodes", "Name_Localised":"Getaggte Verschlüsselungscodes", "Count":33 }, { "Name":"shieldsoakanalysis", "Name_Localised":"Inkonsistente Schildleistungsanalysen", "Count":250 }, { "Name":"scanarchives", "Name_Localised":"Unidentifizierte Scan-Archive", "Count":112 }, { "Name":"disruptedwakeechoes", "Name_Localised":"Atypische FSA-Stör-Aufzeichnungen", "Count":228 }, { "Name":"archivedemissiondata", "Name_Localised":"Irreguläre Emissionsdaten", "Count":65 }, { "Name":"legacyfirmware", "Name_Localised":"Spezial-Legacy-Firmware", "Count":78 }, { "Name":"scrambledemissiondata", "Name_Localised":"Außergewöhnliche verschlüsselte Emissionsdaten", "Count":84 }, { "Name":"encodedscandata", "Name_Localised":"Divergente Scandaten", "Count":30 }, { "Name":"fsdtelemetry", "Name_Localised":"Anormale FSA-Telemetrie", "Count":123 }, { "Name":"wakesolutions", "Name_Localised":"Seltsame FSA-Zielorte", "Count":93 }, { "Name":"emissiondata", "Name_Localised":"Unerwartete Emissionsdaten", "Count":142 }, { "Name":"shieldpatternanalysis", "Name_Localised":"Abweichende Schildeinsatz-Analysen", "Count":78 }, { "Name":"scandatabanks", "Name_Localised":"Scan-Datenbanken unter Verschluss", "Count":68 }, { "Name":"consumerfirmware", "Name_Localised":"Modifizierte Consumer-Firmware", "Count":48 }, { "Name":"symmetrickeys", "Name_Localised":"Offene symmetrische Schlüssel", "Count":24 }, { "Name":"shieldfrequencydata", "Name_Localised":"Verdächtige Schildfrequenz-Daten", "Count":50 }, { "Name":"compactemissionsdata", "Name_Localised":"Anormale kompakte Emissionsdaten", "Count":18 }, { "Name":"adaptiveencryptors", "Name_Localised":"Adaptive Verschlüsselungserfassung", "Count":64 }, { "Name":"encryptionarchives", "Name_Localised":"Atypische Verschlüsselungsarchive", "Count":63 }, { "Name":"dataminedwake", "Name_Localised":"FSA-Daten-Cache-Ausnahmen", "Count":19 }, { "Name":"securityfirmware", "Name_Localised":"Sicherheits-Firmware-Patch", "Count":29 }, { "Name":"embeddedfirmware", "Name_Localised":"Modifizierte integrierte Firmware", "Count":58 }, { "Name":"tg_residuedata", "Name_Localised":"Thargoiden-Rückstandsdaten", "Count":55 }, { "Name":"tg_compositiondata", "Name_Localised":"Materialzusammensetzungsdaten der Thargoiden", "Count":49 }, { "Name":"tg_structuraldata", "Name_Localised":"Thargoiden-Strukturdaten", "Count":49 }, { "Name":"unknownshipsignature", "Name_Localised":"Thargoiden-Schiffssignatur", "Count":37 }, { "Name":"unknownwakedata", "Name_Localised":"Thargoiden-Sogwolkendaten", "Count":55 }, { "Name":"ancienthistoricaldata", "Name_Localised":"Gamma-Muster-Obeliskendaten", "Count":150 }, { "Name":"ancienttechnologicaldata", "Name_Localised":"Epsilon-Muster-Obeliskendaten", "Count":150 }, { "Name":"ancientbiologicaldata", "Name_Localised":"Alpha-Muster-Obeliskendaten", "Count":150 }, { "Name":"ancientlanguagedata", "Name_Localised":"Delta-Muster-Obeliskendaten", "Count":150 }, { "Name":"ancientculturaldata", "Name_Localised":"Beta-Muster-Obeliskendaten", "Count":150 }, { "Name":"classifiedscandata", "Name_Localised":"Geheimes Scan-Fragment", "Count":18 }, { "Name":"hyperspacetrajectories", "Name_Localised":"Exzentrische Hyperraum-Routen", "Count":104 }, { "Name":"guardian_weaponblueprint", "Name_Localised":"Guardian-Waffenbauplanfragment", "Count":4 }, { "Name":"guardian_moduleblueprint", "Name_Localised":"Guardian-Modulbauplanfragment", "Count":7 }, { "Name":"guardian_vesselblueprint", "Name_Localised":"Guardian-Schiffsbauplanfragment", "Count":8 }, { "Name":"tg_shipflightdata", "Name_Localised":"Schiffsflugdaten", "Count":18 }, { "Name":"tg_shipsystemsdata", "Name_Localised":"Schiffssysteme-Daten", "Count":45 } ] }
        "Materials" => {}
        "Cargo" => {}
        "MaterialCollected" => {}
        "Synthesis" => {}
        "EjectCargo" => {}
        "DropItems" => {}
        "LaunchDrone" => {}
        "MiningRefined" => {}
        "ProspectedAsteroid" => {
            //{ "timestamp":"2023-06-05T12:05:12Z", "event":"ProspectedAsteroid", "Materials":[ { "Name":"rutile", "Name_Localised":"Rutil", "Proportion":35.986309 }, { "Name":"Bauxite", "Name_Localised":"Bauxit", "Proportion":13.713245 } ], "Content":"$AsteroidMaterialContent_Low;", "Content_Localised":"Materialgehalt: Niedrig", "Remaining":100.000000 }
        }
        "CargoTransfer" => {}
        "CollectCargo" => {}

        //Mission and Redeeming
        "Missions" => {}
        "MissionAccepted" => {}
        "MissionRedirected" => {}
        "MissionCompleted" => {}
        "RedeemVoucher" => {}
        "Bounty" => {}
        "NpcCrewPaidWage" => {}
        "PayFines" => {}
        "MissionAbandoned" => {}
        "MissionFailed" => {}
        "PayBounties" => {}
        "SellOrganicData" => {}

        //Carrier
        "CarrierStats" => {}
        "CarrierJumpRequest" => {}
        "CarrierTradeOrder" => {}
        "CarrierFinance" => {}
        "CarrierJumpCancelled" => {}
        "CarrierDepositFuel" => {}
        "CarrierDockingPermission" => {}
        "CarrierCrewServices" => {}
        "CarrierModulePack" => {}
        "CarrierBankTransfer" => {}


        //Dropship
        "BookDropship" => {}
        "DropshipDeploy" => {}

        //Wing
        "WingInvite" => {}
        "WingJoin" => {}
        "WingAdd" => {}
        "WingLeave" => {}

        //Crew
        "CrewMemberQuits" => {}
        "CrewMemberRoleChange" => {}
        "CrewMemberJoins" => {}
        "EndCrewSession" => {}

        "SellMicroResources" => {}
        "TradeMicroResources" => {}
        "FuelScoop" => {}
        "ReceiveText" => {}
        "Friends" => {}
        "Scanned" => {}
        "LoadGame" => {}
        "SquadronStartup" => {}
        "Music" => {}
        "CodexEntry" => {}
        "Rank" => {}
        "Progress" => {}
        "Reputation" => {}
        "Statistics" => {}
        "Commander" => {}
        "PowerplaySalary" => {}
        "Powerplay" => {}
        "CommitCrime" => {}
        "DockingDenied" => {}
        "HeatWarning" => {}
        "FactionKillBond" => {}
        "MultiSellExplorationData" => {}
        "SwitchSuitLoadout" => {}
        "MaterialTrade" => {}
        "CommunityGoal" => {}
        "ModuleRetrieve" => {}
        "FetchRemoteModule" => {}
        "SendText" => {}
        "SearchAndRescue" => {}
        "HeatDamage" => {}
        "CommunityGoalReward" => {}
        "NavBeaconScan" => {}
        "USSDrop" => {}
        "Interdicted" => {}
        "Promotion" => {}
        "RepairDrone" => {}
        "DataScanned" => {}
        "DatalinkScan" => {}
        "DatalinkVoucher" => {}
        "CockpitBreached" => {}
        "SystemsShutdown" => {}
        "Screenshot" => {}
        "UpgradeWeapon" => {}
        "PowerplayFastTrack" => {}
        "PowerplayCollect" => {}
        "PowerplayDeliver" => {}
        "BookTaxi" => {}
        "SharedBookmarkToSquadron" => {}
        "MaterialDiscovered" => {}
        "SetUserShipName" => {}
        "FCMaterials" => {}
        "CommunityGoalJoin" => {}
        "SupercruiseDestinationDrop" => {}
        "JetConeBoost" => {}
        "AsteroidCracked" => {}
        "EscapeInterdiction" => {}
        "TechnologyBroker" => {}
        "NavBeaconDetail" => {}


        //Jesus
        "Died" => {}
        "Resurrect" => {}
        "SelfDestruct" => {}

        "Fileheader" => {}
        "Shutdown" => {}
        "None" | "" | _ => {
            let market_id = message["marketId"].as_i64().unwrap();
            let station_name = message["stationName"].to_string();
            let system_name = message["systemName"].to_string();
            let odyssey = message["odyssey"].as_bool().unwrap();
            if !message["ships"].is_null() {
                //ships
                {
                    //language=postgresql
                    let sql = "INSERT INTO station VALUES ($1,$2,$3,$4,$5) ON CONFLICT (market_id,odyssey) DO UPDATE SET timestamp = excluded.timestamp, name = excluded.name, market_id = excluded.market_id, system_name = excluded.system_name;";
                    match client.lock().await.execute(sql, &[
                        &timestamp,
                        &station_name,
                        &market_id,
                        &system_name,
                        &odyssey
                    ]).await {
                        Ok(_) => {}
                        Err(err) => {
                            if !err.to_string().contains("violates foreign key constraint") {
                                panic!("{}", err);
                            }
                        }
                    }
                }
                {
                    //language=postgresql
                    let delete = "DELETE FROM ship WHERE market_id=$1 and odyssey=$2;";
                    client.lock().await.execute(delete, &[
                        &market_id, &odyssey
                    ]).await.unwrap();
                }
                for i in 0..message["ships"].len() {
                    //language=postgresql
                    let insert = "INSERT INTO ship (timestamp, market_id, ship, odyssey) VALUES ($1,$2,$3,$4) ON CONFLICT (market_id, ship, odyssey) DO NOTHING;";
                    match client.lock().await.execute(insert, &[
                        &timestamp,
                        &market_id,
                        &message["ships"][i].to_string(),
                        &odyssey
                    ]).await {
                        Ok(_) => {}
                        Err(err) => {
                            if !err.to_string().contains("violates foreign key constraint") {
                                panic!("{}", err);
                            }
                        }
                    }
                }
            } else {
                if !message["modules"].is_null() {
                    //modules
                    {
                        //language=postgresql
                        let sql = "INSERT INTO station (timestamp, name, market_id, system_name, odyssey) VALUES ($1,$2,$3,$4,$5) ON CONFLICT (market_id,odyssey) DO UPDATE SET timestamp = excluded.timestamp, name = excluded.name, market_id = excluded.market_id, system_name = excluded.system_name, odyssey = excluded.odyssey;";
                        match client.lock().await.execute(sql, &[
                            &timestamp,
                            &station_name,
                            &market_id,
                            &system_name,
                            &odyssey
                        ]).await {
                            Ok(_) => {}
                            Err(err) => {
                                if !err.to_string().contains("violates foreign key constraint") {
                                    panic!("{}", err);
                                }
                            }
                        }
                    }
                    {
                        //language=postgresql
                        let delete = "DELETE FROM module WHERE market_id=$1 and odyssey=$2;";
                        client.lock().await.execute(delete, &[&market_id, &odyssey]).await.unwrap();
                    }
                    for i in 0..message["modules"].len() {
                        //language=postgresql
                        let insert = "INSERT INTO module VALUES ($1,$2,$3,$4) ON CONFLICT (market_id, name, odyssey) DO NOTHING;";
                        match client.lock().await.execute(insert, &[
                            &timestamp,
                            &market_id,
                            &message["modules"][i].to_string(),
                            &odyssey
                        ]).await {
                            Ok(_) => {}
                            Err(err) => {
                                if !err.to_string().contains("violates foreign key constraint") {
                                    panic!("{}", err);
                                }
                            }
                        }
                    }
                } else {
                    if !message["commodities"].is_null() {
                        //commodities
                        {
                            //language=postgresql
                            let sql = "INSERT INTO station VALUES ($1,$2,$3,$4,$5) ON CONFLICT (market_id,odyssey) DO UPDATE SET timestamp = excluded.timestamp, name = excluded.name, market_id = excluded.market_id, system_name = excluded.system_name;";
                            match client.lock().await.execute(sql, &[
                                &timestamp,
                                &station_name,
                                &market_id,
                                &system_name,
                                &odyssey
                            ]).await {
                                Ok(_) => {}
                                Err(err) => {
                                    if !err.to_string().contains("violates foreign key constraint") {
                                        panic!("{}", err);
                                    }
                                }
                            }
                        }
                        {
                            //language=postgresql
                            let delete = "DELETE FROM commodity WHERE market_id=$1 and odyssey=$2;";
                            client.lock().await.execute(delete, &[&market_id, &odyssey]).await.unwrap();
                        }
                        for i in 0..message["commodities"].len() {
                            //language=postgresql
                            let insert = "INSERT INTO commodity VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) ON CONFLICT (name,market_id,odyssey) DO UPDATE SET timestamp = excluded.timestamp, market_id = excluded.market_id, name = excluded.name, buy_price = excluded.buy_price, sell_price = excluded.sell_price,
                                                                                                                     mean_price = excluded.mean_price, demand_bracket = excluded.demand_bracket, stock = excluded.stock, stock_bracket = excluded.stock_bracket, odyssey = excluded.odyssey;";

                            match client.lock().await.execute(insert, &[
                                &timestamp,
                                &market_id,
                                &message["commodities"][i]["name"].to_string(),
                                &message["commodities"][i]["buyPrice"].as_i32().unwrap(),
                                &message["commodities"][i]["sellPrice"].as_i32().unwrap(),
                                &message["commodities"][i]["meanPrice"].as_i32().unwrap(),
                                &message["commodities"][i]["demandBracket"].as_i32(),
                                &message["commodities"][i]["stock"].as_i32().unwrap(),
                                &message["commodities"][i]["stockBracket"].as_i32(),
                                &odyssey
                            ]).await {
                                Ok(_) => {}
                                Err(err) => {
                                    if !err.to_string().contains("violates foreign key constraint") {
                                        panic!("{}", err);
                                    }
                                }
                            }
                        }
                    } else {
                        warn!("Unknown message: {json}");
                    }
                }
            }
        }
    }
}