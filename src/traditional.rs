

pub mod tradhandle {
    use rand::Rng;
    use std::fs;
    #[derive(Debug,Clone)]
    struct TileData {
        enabled: bool,
        xpos: f64,
        ypos: f64,
        offset_x: Option<String>,
        offset_y: Option<String>,
        tile_id: Option<String>,
        tile: bool,
        extra_e: Option<String>,
    }

    fn data_extract(filename: &str) -> Vec<TileData> {
        let data = fs::read_to_string(filename)
            .expect(&format!("Failed to read file {}", filename));
        
        let mut tiles = Vec::new();
        let mut rew: f64 = 0.0;
        let mut reh: f64 = 0.0;
        let mut writestring = String::from("");
        let mut sy: f64 = 0.0;
        let mut sx: f64 = 0.0;
        let mut is_tile = false;
        
        for line in data.lines() {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() != 2 {
                continue;
            }
    
            let key = parts[0].trim();
            let value = parts[1].trim().to_string();
    
            match key {
                "1p" => {
                    let clean = value.trim_matches('"');
                    sx = clean.parse().unwrap_or(0.0);
                },
                "1q" => {
                    let clean = value.trim_matches('"');
                    rew = clean.parse().expect("Failed to parse rew");
                    println!("The number is: {}", rew);
                },
                "1r" => {
                    let clean = value.trim_matches('"');
                    sy = clean.parse().unwrap_or(0.0);
                },
                "1s" => {
                    let clean = value.trim_matches('"');
                    reh = clean.parse().unwrap_or(224.0);
                },
                _ => {},
            }
            if rew != 0.0 && reh != 0.0 {
                println!("startw: {} endw: {}", sx, rew);
                println!("starth: {} endh: {}", sy, reh);
                break;
            }
        }
        
        // Iterate over the range with a step
        let step = 16.0; // Define the step size
        for y in (sy as usize / 224..((sy + reh) as usize / 224)).map(|y| y as f64) {
            writestring = format!("{}\n POSITION {}",writestring,y);
            for x in (sx as usize / 16..((sx + rew) as usize / 16)).map(|x| x as f64) {
                let mut current_tile = TileData {
                    enabled: false,
                    xpos: x * 16.0,
                    ypos: y * 16.0,
                    offset_x: None,
                    offset_y: None,
                    tile_id: None,
                    tile: false,
                    extra_e: None,
                };
    
                for line in data.lines() {
                    let parts: Vec<&str> = line.split('=').collect();
                    if parts.len() != 2 {
                        continue;
                    }
    
                    let key = parts[0].trim();
                    let value = parts[1].trim().to_string();
    
                    match key.chars().next() {
                        Some('a') => {
                           
                        },
                        Some('k') => current_tile.offset_x = Some(value),
                        Some('j') => current_tile.offset_y = Some(value),
                        Some('i') => {
                            current_tile.tile = value.contains("1");
                            if current_tile.tile {
                                writestring.push('▣');
                            } else {
                                writestring.push('_');
                            }
                            is_tile = true;
                        },
                        Some('e') => current_tile.extra_e = Some(value),
                        Some('d') => current_tile.tile_id = Some(value),
                        _ => {
                            writestring.push('_');
                        },
                    }
    
                    if current_tile.enabled && current_tile.tile_id.is_some() && is_tile {
                        tiles.push(current_tile.clone());
                    } else {
                        writestring.push('_');
                    }
                }
    
                if current_tile.enabled || current_tile.tile_id.is_some() {
                    tiles.push(current_tile);
                }
            }
            
        }
    
        for i in 0..tiles.len() {
            println!(
                "enabled: {:?}, x: {:?}, y: {:?}, tile_id: {:?}",
                tiles[i].enabled, tiles[i].xpos, tiles[i].ypos, tiles[i].tile_id
            );
        }
    
        fs::write("guide.lvg", writestring.clone()).expect("failed to write guide"); // write all data to the mmlv file.
        tiles
    }


    fn handle_weapon(mut text: String) -> String { //weapon system
        //will force a weapon onto slot zero because 90% of the levels i generated didnt have the player have a default wpn
        let dfwpn_rng: u64 = rand::thread_rng().gen_range(0..105);
        text = format!("{}\"{}\"", format!("{}\n1k{}=",text,0), dfwpn_rng);
        for i in 1..12 {
            let vartouse: u64 = rand::thread_rng().gen_range(0..25);
            if vartouse <= 10 {
                text = format!("{}\"{}\"", format!("{}\n1k{}=",text,i), -1);
            } else {
                let rand_num: u64 = rand::thread_rng().gen_range(0..105);
                text = format!("{}\"{}\"", format!("{}\n1k{}=",text,i), rand_num);
            }
        }
        text
    }

    fn handle_music(mut text: String) -> String { //selects a level song
        let mut category: u64 = rand::thread_rng().gen_range(0..10);
        loop {
            
            if category == 11 || category == 12 {
                category = rand::thread_rng().gen_range(0..10);
                continue;
            }
            else {
                let song: u64 = rand::thread_rng().gen_range(0..7);
                text = format!("{}1l=\"{}\"\n1m=\"{}\"",text,category,song);
                break;

            }

        }
        

        text
    }

    fn handle_tiling(mut text: String, level_length: i64, verttiles: Vec<i64>) -> (String, Vec<i64>) { //adds tiles
        let mut pointchecker = 0;
        let mut screeny = 0;
        let tile_id: u64 = rand::thread_rng().gen_range(0..1315);
        let mut vecheight = Vec::new();  
        #[allow(unused_assignments)]
        let mut can_proceed = false;
        let mut vecheighttrack = 0;
        let mut height = rand::thread_rng().gen_range(1..13);
        for i in 0..level_length / 16 {
            
                
                if pointchecker < verttiles.len() {
                    if i-1 == verttiles[pointchecker]/16 && i * 256 != level_length {
                        for j in 1..height {
                            
                            vecheight.push(height); 
                            vecheighttrack += 1;
                            text = format!(
                                "{}a{},{}=\"1\"\ne{},{}=\"{}\"\ni{},{}=\"1\"\nj{},{}=\"1\"\nk{},{}=\"1\"\n",
                                text, i-1 * 16, (screeny)+224 - j * 16, i-1 * 16, (screeny)+224 - j * 16, tile_id,
                                i-1 * 16, (screeny)+224 - j * 16, i-1 * 16, (screeny)+224 - j * 16, i-1 * 16, (screeny)+224 - j * 16
                            );
                        }
                        screeny+=224;
                        pointchecker+=1;
                        
                        println!("{screeny}");
                    }
                
                
            }
            if vecheighttrack == 0 {
                height = rand::thread_rng().gen_range(1..13);
                can_proceed = true;
            } else {
                loop {
                    let temp_height =  rand::thread_rng().gen_range(1..13);
                    if temp_height > vecheight[vecheighttrack - 1] + 3 || temp_height > 12 {
                            continue;
                        } else {
                            height = temp_height;
                            can_proceed = true;
                            break;
                        }
                    }
                }
        
                if can_proceed {
                    
                    for j in 1..height {
                        vecheight.push(height); 
                        vecheighttrack += 1;
                    
                        text = format!(
                            "{}a{},{}=\"1\"\ne{},{}=\"{}\"\ni{},{}=\"1\"\nj{},{}=\"1\"\nk{},{}=\"1\"\n",
                            text, i * 16, (screeny)+224 - j * 16, i * 16, (screeny)+224 - j * 16, tile_id,
                            i * 16, (screeny)+224 - j * 16, i * 16, (screeny)+224 - j * 16, i * 16, (screeny)+224 - j * 16
                        );
                        
                    }
                }
            }
        
            (text, vecheight)  
        }

    fn handle_abilities(mut text: String) -> String { //changes default level abilities
        let can_charge; //can megaman charge buster?
        let can_charge_rng = rand::thread_rng().gen_range(0..4);
        if can_charge_rng == 4 {can_charge = 0;}
        else {
            can_charge = 1;
        }
        let charge_rng = rand::thread_rng().gen_range(4..6); //what charge shot will megaman use?
        let slide_rng = rand::thread_rng().gen_range(0..5); //can megaman slide?
        text = format!("{}\n1b=\"{}\"\n1c=\"{}\"\n1d=\"{}\"\n",text,slide_rng,can_charge,charge_rng); //mega's abilities

        let can_strike; //can protoman use proto strike (all shots are charge shots)
        let can_strike_rng = rand::thread_rng().gen_range(0..4); 
        if can_strike_rng == 4 {can_strike = 0;}
        else {
            can_strike = 1;
        }
        let dd_rng = rand::thread_rng().gen_range(0..1); //does proto man take double damage?
        let dj_rng = rand::thread_rng().gen_range(0..1); //can bass double jump?
        let dr_rng = rand::thread_rng().gen_range(0..1); //can roll dodge roll?
        let cb_rng = rand::thread_rng().gen_range(0..1); //can roll charge her broom?
        text = format!("{}1ba=\"{}\"\n1ca=\"{}\"\n1bb=\"{}\"\n1cb=\"{}\"\n1cc=\"{}\"\n",text,dd_rng,can_strike,dj_rng,dr_rng,cb_rng); //proto and bass's abilities
        
        text
    }
    pub fn file_write() {
        let bgcount = rand::thread_rng().gen_range(0..732);
        let length: i64 = rand::thread_rng().gen_range(1..50)*256;
        //screen trans
        let mut transpoints = Vec::new();
        for c in 0..length/256 {
            
            let transition = rand::thread_rng().gen_bool(1.0 / 4.0);
            if transition == true && c * 256 != length-256{
                
                transpoints.push(c*256);
            
                
            }
        }
        let mut pointchecker = 0;
        let mut screeny = 0;

        //naming
        let names = Vec::from(["remastered","cut man","intro stage","level pack", "kaizo", "1-5", "protovania", "2023 revamped","roll","tutorial","wily stage","6","woman","man","mega man 12", "mega man 13", "mega man 10", "enker", "GB", "NES", "remake","challenge", "recreated", "recreation","demake","7","8","boss rush","crystal gate","{rand::thread_rng().gen_range(1..21)}","kazoo","kiazo","fangame","yellow devil","nico evaluates","rockman and forte","the sequel","1_8_0","1_7_5","1_6_0","puzzle","neo cutman","contest","i wanna kill megaman","force beam","gimmick","contraption","illegal","factory","cutmna","hardman","concept", "mega man x","zero","mega man maker x","community maker","fortnite","joe biden","strike man","megaman","protoman","bass","roll","super hard","impossible","worlds hardest","easy","traditional","megaman 2","magnet","pluto","saturn","stardroid","battan","cossack","stage","airship","fire base","dark man","4","3","2","1","big pets","Ryu","sea",
    "v2","v3","v4","passage","entrance","skull","castle","gun","nrs","vui","feeber","example level","prontoman","mega man","rockman","electro guard","speedrun","tech","glitch","what","a","leafshield","bielles","mmmx","modded","-","-","-","-","-","-","-","-","-","-","-","-","wow","hard","ez","meka snack","go fast","apology level","b.dash","vs","the level","- ultimate edition","deluxe edition","and bass","dark man 5","fortress","castle","cut","intro","stage","12","13","i","wanna","kill","guard","if it was good","improvement","halloween","christmas","walk","finish","line","death","temple","DWN","dead","man","e","ballade","punk","gate","spam","burner man","big fish","stage","pronto man","heat ladder","quint","sunstar","palace","megamix","bpss","cossack","wily","steam man","meme","dead","bals"]);
        let mut name = String::new();
        for _ in 0..rand::thread_rng().gen_range(2..7) {
            name = format!("{}{} ",name,names[rand::thread_rng().gen_range(1..names.len()-1)]);

        }

        //init
        let mut contents = String::from("[Level]");
        //weapons
        let binding = handle_weapon(contents.clone());
        contents = binding;
        //general things
        let mugshot = rand::thread_rng().gen_range(1..41); //boss mugshot id
        contents = format!("{}\n0v=\"1.8.5.2\"\n1a=\"{}\"\n4a=\"MMMRNG\"\n4b=\"{}\"\n0a=\"000000\"\n1p=\"0\"\n1q=\"{}\"\n1r=\"0\"\n1s=\"4480\"\n1bc=\"1\"\n1f=\"{}\"\n1e=\"{}\"\n", contents,name,rand::thread_rng().gen_range(0..161),length,mugshot,rand::thread_rng().gen_range(0..51)); //adds general level info
        //musica
        let binding = handle_music(contents.clone());
        contents = binding;
        //player abilities
        let binding = handle_abilities(contents.clone());
        contents = binding;

        //activate sections and add backgrounds
        for i in -1..length/256 {
            
            if pointchecker < transpoints.len() {
                if i-1 == transpoints[pointchecker]/256 {
                    if i != -1 {
                        screeny+=224;
                        pointchecker+=1;
                        contents = format!("{}2a{},{}=\"1\"\n",contents,(i-1)*256,screeny);
                        //add bg
                        
                        contents = format!("{}2d{},{}=\"{}\"\n",contents,(i-1)*256,screeny,bgcount);
                        println!("{screeny}");
                    }
                    else {
                        screeny+=224;
                        pointchecker+=1;
                        contents = format!("{}2a{},{}=\"1\"\n",contents,(i)*256,screeny);
                        //add bg
                        
                        contents = format!("{}2d{},{}=\"{}\"\n",contents,i*256,screeny,bgcount);
                        println!("{screeny}");

                    }
                }
            }
            if i != -1 {
                contents = format!("{}2a{},{}=\"1\"\n",contents,i*256,screeny);
                //add bg
                contents = format!("{}2d{},{}=\"{}\"\n",contents,i*256,screeny,bgcount);
                println!("section at {},{} activated.",i*256,screeny);
            }
        }

        //TILING!!!!!!!!!!! 
        let binding = handle_tiling(contents.clone(),length,transpoints.clone());
        contents = binding.0;
        let vecheights: Vec<i64> = binding.1;
        let objpoints = transpoints.clone();

        //oh and object placements
        //let binding = handle_megaman(contents.clone(),vecheights.clone());
        //contents = binding.0;
        //let binding = handle_objs(contents.clone(),vecheights.clone(),length,binding.1,binding.2,objpoints);
        //contents = binding;

        //handle boss placement
        let mut bossid;
        loop {
            bossid = rand::thread_rng().gen_range(1..68);
            if bossid != 33 && bossid != 0 && bossid != 34 && bossid != 1 && bossid != 55 && bossid != 56 && bossid != 9 &&  bossid != 57 && bossid != 68 && bossid != 36 && bossid != 59 && bossid != 60 { //disables boss suppressor, boss doors, and kamegoro generators as they softlock the player and arent bosses anyways. also disables boobeam trap and gemini man to prevent crashes
                break;

            }
            else {
                continue;

            }
        }
        //let binding  = handle_boss(contents.clone(),bossid,vecheights.clone(),length.clone(),transpoints.clone());
        //contents = binding;


        fs::write("level.mmlv", contents.clone()).expect("failed to write mmlv"); //write all data to the mmlv file.
        data_extract("frosty.mmlv");
        
    }

}
