

pub mod tradhandle {
    use rand::Rng;
    use std::fs;
    #[derive(Debug,Clone)]
    struct TileData {
        enabled: bool,
        xpos: i64,
        ypos: i64,
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
        let mut rew = 256/16;
        let mut reh = (1*224)/16;
        let mut writestring = String::from("");

            for y in 0..reh {
                let buffer = format!("{}\n",writestring.clone());
                writestring = buffer;
                for x in 0..rew {
                    let mut current_tile = TileData {
                        enabled: false,
                        xpos: x * 16,
                        ypos: y * 16,
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
                                if current_tile.tile == true {

                                    let buffer = format!("{}1",writestring.clone());
                                    writestring = buffer;
                                }
                                else {
                                    let buffer = format!("{}0",writestring.clone());
                                    writestring = buffer;
                                }
                            },
                            Some('e') => current_tile.extra_e = Some(value),
                            Some('d') => current_tile.tile_id = Some(value),
                            _ => {
                                writestring = format!("{}0",writestring);
                                
                            },
                        }

                        if current_tile.enabled && current_tile.tile_id.is_some() {
                            tiles.push(current_tile.clone());
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
        println!("{}",writestring);
        fs::write("guide.lvg", writestring.clone()).expect("failed to write guide"); //write all data to the mmlv file.
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
        

    fn handle_megaman(mut text: String, levelheights: Vec<i64>) -> (String,i64,i64) {
        let xpos = (rand::thread_rng().gen_range(1..7) * 16) as usize;
        let ypos = 224-(((levelheights[xpos/16]))*16)-16;
        let playerid = rand::thread_rng().gen_range(0..4);
        println!("x and y is {},{}. player id is {}.", xpos,ypos,playerid);

        text = format!("{}a{},{}=\"1\"\nb{},{}=\"-1\"\nc{},{}=\"1\"\nd{},{}=\"4\"\ne{},{}=\"{}\"\n",text,xpos,ypos,xpos,ypos,xpos,ypos,xpos,ypos,xpos,ypos,playerid); //basic stuffs
        text = format!("{}f{},{}=\"0\"\ng{},{}=\"0\"\nh{},{}=\"1\"\ni{},{}=\"0\"\n",text,xpos,ypos,xpos,ypos,xpos,ypos,xpos,ypos); //other properties

        (text,xpos.try_into().unwrap(),ypos)
    }


    fn handle_objs(mut text: String, levelheights: Vec<i64>, lvllength: i64, megax: i64, megay: i64, verttiles: Vec<i64>) -> String {
        let count = rand::thread_rng().gen_range(0..lvllength / 32);
        let objectids = vec![
            rand::thread_rng().gen_range(0..237),
            rand::thread_rng().gen_range(0..237),
            rand::thread_rng().gen_range(0..237),
            rand::thread_rng().gen_range(0..237),
            rand::thread_rng().gen_range(0..237),
        ];
        #[allow(unused_assignments)]
        let mut screeny = 0;
        println!("enemy count: {}", count);
        let l: usize = lvllength.try_into().unwrap();
        println!("{}", l);

        for _ in 0..count {
            let mut xpos = rand::thread_rng().gen_range(1..=(l / 16));
            xpos = ((xpos + 7) / 16) * 16;
            
            if xpos >= l {
                continue;
            }

            // reset screeny for each xpos
            screeny = 0;
            for &tile in &verttiles {
                let tile_usize = tile as usize;
                
                if xpos*16 > tile_usize {
                    screeny += 224;
                    
                }
            }

            let mut ypos = 224 - ((levelheights[xpos] * 16) as i64);
            ypos += screeny;
            if xpos as i64 != megax && ypos != megay {
                text = format!(
                    "{}a{},{}=\"1\"\nb{},{}=\"1\"\nc{},{}=\"1\"\nd{},{}=\"5\"\ne{},{}=\"{}\"\n",
                    text, xpos * 16, ypos, xpos * 16, ypos, xpos * 16, ypos, xpos * 16, ypos, xpos * 16, ypos, objectids[rand::thread_rng().gen_range(0..4)]
                );
                text = format!(
                    "{}f{},{}=\"0\"\ng{},{}=\"0\"\nh{},{}=\"1\"\ni{},{}=\"0\"\n",
                    text, xpos * 16, ypos, xpos * 16, ypos, xpos * 16, ypos, xpos * 16, ypos
                );
                println!("x is {}. y is {}.", xpos, ypos);
            }
        }

        text
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

    fn handle_boss(mut text: String, bossid: u64, _levelheights: Vec<i64>,length: i64, transpoints: Vec<i64>) -> String {
        let mut screeny = 0;
        let mut pointchecker = 0;
        
        let l: usize = length.try_into().unwrap(); 
        let xpos = (l - 256) + (rand::thread_rng().gen_range(7..16)) * 16 as usize;

        let ypos = 224-(rand::thread_rng().gen_range(7..10))*16;
        
        for i in -1..length/256 {
            
            if pointchecker < transpoints.len() {
                if i-1 == transpoints[pointchecker]/256 {
                    screeny+=224;
                    pointchecker+=1;
                    println!("{screeny}");
                }
            }
        }

        /* 
        BOSS INI CODE:
        o16,16="9999.000"
        e16,16="2.000"
        d16,16="8.000"
        b16,16="-1.000"
        a16,16="1.000"

        1xcX - x pos
        1ycX - y pos
        1gaX - true
        1gX - 1
        1haX - false
        1hX - 0
        1iX - false
        1jX - 0

        1nX - boss msuic - set to 1 for mm2 boss theme
        1oX - ost type, will be set to 0
        */

        
        text = format!("{}a{},{}=\"1\"\nb{},{}=\"1\"\nc{},{}=\"1\"\nd{},{}=\"8\"\ne{},{}=\"{}\"\n",text,xpos,ypos+screeny,xpos,ypos+screeny,xpos,ypos+screeny,xpos,ypos+screeny,xpos,ypos+screeny,bossid); //basic stuffs
        text = format!("{}1xc0=\"{}\"\n1yc0=\"{}\"\n1ga0=\"1\"\n1g0=\"1\"\n1ha0=\"0\"\n1h0=\"1\"\n1i0=\"0\"\n1j0=\"0\"\n1n0=\"1\"\n1o0=\"0\"\n",text,xpos,ypos+screeny);
        
        println!("boss category is 8. boss id is {}.", bossid);
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
        let binding = handle_megaman(contents.clone(),vecheights.clone());
        contents = binding.0;
        let binding = handle_objs(contents.clone(),vecheights.clone(),length,binding.1,binding.2,objpoints);
        contents = binding;

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
        let binding  = handle_boss(contents.clone(),bossid,vecheights.clone(),length.clone(),transpoints.clone());
        contents = binding;


        fs::write("level.mmlv", contents.clone()).expect("failed to write mmlv"); //write all data to the mmlv file.
        data_extract("metal.mmlv");
        
    }

}
