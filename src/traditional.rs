pub mod tradhandle {
    use rand::{thread_rng, Rng};
    use std::fs;
    use std::fs::read_to_string;
    use std::path::Path;

    fn repeat<F: FnMut()>(mut f: F, times: u64) {
        for _ in 0..times {
            f()
        }
    }

    fn rows_generate() -> Vec<bool> {
        let mut rows = Vec::new();
        repeat(
            || {
                rows.push(thread_rng().gen_bool(7.0 / 10.0));
            },
            15,
        );
        rows
    }

    fn handle_tiling(
        mut text: String,
        level_length: i64,
        verttiles: Vec<i64>,
    ) -> (String, Vec<i64>) {
        let mut pointchecker = 0;
        let mut screen_y = 0;
        let mut vec_height: Vec<i64> = Vec::new();
        let mut r = rows_generate();
        for i in 0..(level_length / 16) {
            if pointchecker < verttiles.len() && i * 16 >= verttiles[pointchecker] {
                vec_height.push(screen_y);
               
                for k in 0..16 {
                    for j in 0..14 {
                        if r[j] {
                            let x = i * 16 + (k * 16);
                            let y = screen_y + (j * 16) as i64;
                            println!("x: {}, y: {}, tileset index: 1", x, y);
                            text.push_str(&format!(
                                "a{},{}=\"1\"\ne{},{}=\"1\"\ni{},{}=\"1\"\nj{},{}=\"1\"\nk{},{}=\"1\"\n",
                                x, y, x, y, x, y, x, y, x, y,
                            ));
                        }
                    }
                }
                screen_y += 224;
                pointchecker += 1;
            }
            for j in 0..14 {
                if r[j] {
                    let x = i * 16;
                    let y = screen_y + (j * 16) as i64;
                    println!("x: {}, y: {}, tileset index: 1", x, y);
                    text.push_str(&format!(
                        "a{},{}=\"1\"\ne{},{}=\"1\"\ni{},{}=\"1\"\nj{},{}=\"1\"\nk{},{}=\"1\"\n",
                        x, y, x, y, x, y, x, y, x, y,
                    ));
                }
            }
        }
       
        (text, vec_height)
    }

    trait FloorTo: Sized {
        fn floor_to(&self, num: i64) -> i64;
    }

    impl FloorTo for i64 {
        fn floor_to(&self, num: i64) -> i64 {
            self - (self % num)
        }
    }
    trait Convert {
        fn as_int(&self) -> u64;
    }
    impl Convert for bool {
        fn as_int(&self) -> u64 {
            if *self {
                1
            } else if !(*self) {
                0
            } else {
                1
            }
        }
    }
    fn choose<T: Clone>(ops: Vec<T>) -> T {
        let i = ops.len();
        let rng = rand::thread_rng().gen_range(0..i);
        ops[rng].clone()
    }

    fn handle_music(mut text: String) -> String {
        //selects a level song
        let mut category: u64 = rand::thread_rng().gen_range(0..10);
        loop {
            if category == 11 || category == 12 {
                category = rand::thread_rng().gen_range(0..10);
                continue;
            } else {
                let song: u64 = rand::thread_rng().gen_range(0..7);
                text = format!("{}1l=\"{}\"\n1m=\"{}\"", text, category, song);
                break;
            }
        }

        text
    }

    fn handle_boss(contents: String, bossid: u64, level_length: i64, screeny: u64) -> String {
        let holy_shit_sans_undertale = rand::thread_rng().gen_bool(1.0 / 69.0); //one in 69 chance to choose megalovania as the boss music. (text editing only feature which can be accessed by setting a boss theme id to 69)
        let bossmusic = rand::thread_rng().gen_range(1..20);
        let mut mus = 0;
        match holy_shit_sans_undertale {
            true => {
                mus = 69;
            }
            false => {
                mus = bossmusic;
            }
        }
        let mut bossx = 0;
        let mut bossy = 0;
        /*
        The following bosses don't have 2x2 hitboxes:
        hard man: 3x3
        stone man: 2x3
        charge man: 2x3
        wind man: 3x3
        shade/spring man: 2x3
        astro man: 3x3
        astro man: 3x3
        blast man: 3x3? (i think)
        bounce man: either 4x4, 5x5, 4x5, or 3x4
        splash woman: 2x3
        pirate man: 2x3
        freeze man: 2x3
        ALL BOOBEAM TYPES (including controller): 1x1

        the following bosses require special aid from other gimmicks and thus cant be fought (maybe not big pets or kamegoro tho)
        boobeam
        kamegoro maker
        big pets
        yellow devil

        the following bosses i have zero god damn idea what their hitbox size is:
        wily machine 4
        wily machine 6
        cossack catcher
        yellow devil
        big pets
        */

        bossx = level_length - 48;
        bossy = (screeny + 224) - 32;

        let mut text = format!(
            "{}a{},{}=\"1\"\nb{},{}=\"1\"\nc{},{}=\"1\"\nd{},{}=\"8\"\ne{},{}=\"{}\"\n",
            contents, bossx, bossy, bossx, bossy, bossx, bossy, bossx, bossy, bossx, bossy, bossid
        );
        text = format!("{}1xc0=\"{}\"\n1yc0=\"{}\"\n1ga0=\"1\"\n1g0=\"1\"\n1ha0=\"0\"\n1h0=\"1\"\n1i0=\"0\"\n1j0=\"0\"\n1n0=\"{}\"\n1o0=\"0\"\n",text,bossx,bossy, mus);
        println!("boss at {},{}. id is {}", bossx, bossy, bossid);
        text
    }

    fn handle_weapon(mut text: String) -> String {
        //weapon system
        //will force the mega buster onto slot zero since this IS a traditional lvl
        text = format!("{}\n1k0=\"0\"", text);
        for i in 1..12 {
            //picks a random wpn id from versions 1.0 to 1.8.5.2, although older versions of the rng only supporterd 1.0 to 1.6.3
            let rand_num: u64 = rand::thread_rng().gen_range(1..105); //mega buster is removed from the weapon pool, unlike classic mode
            text = format!("{}\"{}\"", format!("{}\n1k{}=", text, i), rand_num);
            //unlike classic, ALL slots will be filled
        }
        text
    }

    fn handle_abilities(mut text: String) -> String {
        //changes default level abilities
        let can_charge; //can megaman charge buster?
        let can_charge_rng = rand::thread_rng().gen_bool(8.0 / 10.0); // 8/22/24: updated rng
        if can_charge_rng {
            can_charge = 0;
        } else {
            can_charge = 1;
        }
        let charge_rng = rand::thread_rng().gen_range(4..7); //what charge shot will megaman use? 8/22: fixed the rng, now mm6 charge shots can be used
        let slide_rng = rand::thread_rng().gen_bool(9.0 / 10.0).as_int(); //can megaman slide? 8/22: updated rng
        text = format!(
            "{}\n1b=\"{}\"\n1c=\"{}\"\n1d=\"{}\"\n",
            text, slide_rng, can_charge, charge_rng
        ); //mega's abilities

        let can_strike; //can protoman use proto strike (all shots are charge shots)
        let can_strike_rng = rand::thread_rng().gen_range(0..4);
        if can_strike_rng == 4 {
            can_strike = 0;
        } else {
            can_strike = 1;
        }
        let dd_rng = rand::thread_rng().gen_range(0..1); //does proto man take double damage?
        let dj_rng = rand::thread_rng().gen_range(0..1); //can bass double jump?
        let dr_rng = rand::thread_rng().gen_range(0..1); //can roll dodge roll?
        let cb_rng = rand::thread_rng().gen_range(0..1); //can roll charge her broom?
        text = format!(
            "{}1ba=\"{}\"\n1ca=\"{}\"\n1bb=\"{}\"\n1cb=\"{}\"\n1cc=\"{}\"\n",
            text, dd_rng, can_strike, dj_rng, dr_rng, cb_rng
        ); //proto and bass's abilities

        text
    }

    pub fn file_write() {
        let bgcount = rand::thread_rng().gen_range(0..732);
        let length: i64 = rand::thread_rng().gen_range(17..31) * 256;
        
        //screen trans
        let mut transpoints = Vec::new();
        for c in 0..length / 256 {
            let transition = rand::thread_rng().gen_bool(1.0 / 6.9);
            if transition && c * 256 != length - 256 {
                transpoints.push(c * 256);
            }
        }
        let mut pointchecker = 0;
        let mut screeny = 0;

        //naming
        fn read_lines(filename: &str) -> Vec<String> {
            if Path::new("names.txt").exists() {
                read_to_string(filename)
                    .unwrap()
                    .lines()
                    .map(String::from)
                    .collect()
            } else {
                vec![
                    "ERROR".to_string(),
                    "NO NAMES FOUND".to_string(),
                    "ERROR".to_string(),
                    "SYSTEM ERROR".to_string(),
                    "PLEASE GET NAMES".to_string(),
                ]
            }
        }

       
        let names = read_lines("names.txt");
        let mut name = String::new();

        if thread_rng().gen_bool(1.0/10.0) {
            name = format!(
                "Mega Man {} - {}s Fortress Stage {}",
                names[rand::thread_rng().gen_range(1..names.len() - 1)],
                names[rand::thread_rng().gen_range(1..names.len() - 1)],
                thread_rng().gen_range(0..70) //picks a random number from 1 to 69. not 70, that's unfunny.
            );
        } else {
            let female = rand::thread_rng().gen_bool(1.0 / 4.0); //gender decider
            let mut fstring = String::from("Man");
            if female {
                fstring = String::from("Woman");
            }

            name = format!(
                "Mega Man {} - {} {}s Stage",
                names[rand::thread_rng().gen_range(1..names.len() - 1)],
                names[rand::thread_rng().gen_range(1..names.len() - 1)],
                fstring
            );
        }
        
        //init
        let mut contents = String::from("[Level]");
        //weapons
        let binding = handle_weapon(contents.clone());
        contents = binding;
        //general things
        let mugshot = rand::thread_rng().gen_range(1..41); //boss mugshot id
        contents = format!("{}\n0v=\"1.9.0\"\n1a=\"{}\"\n4a=\"MMMRNG\"\n4b=\"{}\"\n0a=\"000000\"\n1p=\"0\"\n1q=\"{}\"\n1r=\"0\"\n1s=\"4480\"\n1bc=\"1\"\n1f=\"{}\"\n1e=\"{}\"\n", contents,name,rand::thread_rng().gen_range(0..161),length,mugshot,rand::thread_rng().gen_range(0..51)); //adds general level info
                                                                                                                                                                                                                                                                                          //musica
        let binding = handle_music(contents.clone());
        contents = binding;
        //player abilities
        let binding = handle_abilities(contents.clone());
        contents = binding;

        //activate sections and backgrounds
        for i in -1..length / 256 {
            if pointchecker < transpoints.len() && i - 1 == transpoints[pointchecker] / 256 {
                if i != -1 {
                    screeny += 224;
                    pointchecker += 1;
                    contents = format!("{}2a{},{}=\"1\"\n", contents, (i - 1) * 256, screeny);
                    //add bg

                    contents = format!(
                        "{}2d{},{}=\"{}\"\n",
                        contents,
                        (i - 1) * 256,
                        screeny,
                        bgcount
                    );
                    println!("{screeny}");
                } else {
                    screeny += 224;
                    pointchecker += 1;
                    contents = format!("{}2a{},{}=\"1\"\n", contents, (i) * 256, screeny);
                    //add bg

                    contents = format!("{}2d{},{}=\"{}\"\n", contents, i * 256, screeny, bgcount);
                    println!("{screeny}");
                }
            }
            if i != -1 {
                contents = format!("{}2a{},{}=\"1\"\n", contents, i * 256, screeny);
                //add bg
                contents = format!("{}2d{},{}=\"{}\"\n", contents, i * 256, screeny, bgcount);
                println!("section at {},{} activated.", i * 256, screeny);
            }
        }

        //TILING!!!!!!!!!!!
        let binding = handle_tiling(contents.clone(), length, transpoints.clone());
        contents = binding.0;
        let vecheights: Vec<i64> = binding.1;
        let objpoints = transpoints.clone();

        //oh and object placements
        //let binding = handle_megaman(contents.clone(), vecheights.clone());
        //contents = binding.0;
        /*
        let binding = handle_objs(
            contents.clone(),
            vecheights.clone(),
            length,
            binding.1,
            binding.2,
            objpoints,
        );
        contents = binding;
        */
        //handle boss placement
        let mut bossid;
        loop {
            bossid = rand::thread_rng().gen_range(1..68);
            if bossid != 33
                && bossid != 0
                && bossid != 34
                && bossid != 1
                && bossid != 55
                && bossid != 56
                && bossid != 9
                && bossid != 57
                && bossid != 68
                && bossid != 36
                && bossid != 59
                && bossid != 60
            {
                //disables boss suppressor, boss doors, and kamegoro generators as they softlock the player and arent bosses anyways. also disables boobeam trap and gemini man to prevent crashes
                break;
            } else {
                continue;
            }
        }
        //let binding = handle_boss(contents.clone(), bossid, length, screeny);
        //contents = binding;

        fs::write("level.mmlv", contents.clone()).expect("failed to write mmlv");
        //write all data to the mmlv file.
    }
}
