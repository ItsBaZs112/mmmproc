#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unreachable_patterns)]
#[allow(unused_must_use)]
#[allow(unused_imports)]
pub mod tradhandle {

    fn choose<T: Clone>(ops: Vec<T>) -> T {
        let i = ops.len();
        let rng = rand::thread_rng().gen_range(0..i);
        ops[rng].clone()
    }

    use rand::{thread_rng, Rng};
    use std::fs::read_to_string;
    use std::{fs, vec};

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

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum MetaTile {
        Full,
        StairLeft,
        StairRight,
        InvertedStairL,
        InvertedStairR,
        LineTop,
        LineBottom,
        LineLeft,
        LineRight,
        NoTile,
    }

    impl MetaTile {
        fn from(tile: MetaTile, tst: u64, x: u64, y: u64) -> Vec<TileData> {
            fn create_tile(enabled: bool, xpos: u64, ypos: u64, tile_id: u64) -> TileData {
                TileData {
                    enabled,
                    xpos,
                    ypos,
                    offset_x: 1,
                    offset_y: 1,
                    tile_id,

                    tile: true,
                }
            }

            match tile {
                MetaTile::Full => vec![
                    create_tile(true, x, y, tst),
                    create_tile(true, x + 16, y, tst),
                    create_tile(true, x + 16, y + 16, tst),
                    create_tile(true, x, y + 16, tst),
                ],
                MetaTile::InvertedStairL => vec![
                    create_tile(true, x, y, tst),
                    create_tile(true, x + 16, y, tst),
                    create_tile(true, x, y + 16, tst),
                ],
                MetaTile::StairLeft => vec![
                    create_tile(true, x + 16, y, tst),
                    create_tile(true, x + 16, y + 16, tst),
                    create_tile(true, x, y + 16, tst),
                ],
                MetaTile::StairRight => vec![
                    create_tile(true, x, y, tst),
                    create_tile(true, x + 16, y, tst),
                    create_tile(true, x, y + 16, tst),
                ],
                MetaTile::InvertedStairR => vec![
                    create_tile(true, x, y, tst),
                    create_tile(true, x + 16, y, tst),
                    create_tile(true, x + 16, y + 16, tst),
                ],
                MetaTile::LineTop => vec![
                    create_tile(true, x, y, tst),
                    create_tile(true, x + 16, y, tst),
                ],
                MetaTile::LineBottom => vec![
                    create_tile(true, x + 16, y + 16, tst),
                    create_tile(true, x, y + 16, tst),
                ],
                MetaTile::LineLeft => vec![
                    create_tile(true, x, y, tst),
                    create_tile(true, x, y + 16, tst),
                ],
                MetaTile::LineRight => vec![
                    create_tile(true, x + 16, y, tst),
                    create_tile(true, x + 16, y + 16, tst),
                ],
                MetaTile::NoTile => Vec::new(),
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct TileData {
        //this will be used for genrating realistic, megaman-like tile data, complete with an autotile system
        #[allow(dead_code)]
        enabled: bool,
        xpos: u64,
        ypos: u64,
        #[allow(dead_code)]
        offset_x: u64,
        #[allow(dead_code)]
        offset_y: u64,
        tile_id: u64,
        #[allow(dead_code)]
        tile: bool,
    }

    impl TileData {
        #[allow(dead_code)]
        #[allow(unused_variables)]
        #[allow(unused_assignments)]
        fn autotile_prep(tiles: &TileData, data: &Vec<TileData>) -> TileData {
            let mut ml = false;
            let mut tm = false;
            let mut mr = false;
            let mut bm = false;
            for tile in data.iter() {
                if tile.enabled {
                    if let Some(y) = tiles.ypos.checked_sub(16) {
                        if tile.ypos == tiles.ypos - 16 {
                            tm = true;
                        }
                    }
                    if tile.ypos == tiles.ypos + 16 {
                        bm = true;
                    }
                    if let Some(x) = tiles.xpos.checked_sub(16) {
                        if tile.xpos == tiles.xpos - 16 {
                            ml = true;
                        }
                    }
                    if tile.xpos == tiles.xpos + 16 {
                        mr = true;
                    }
                }
            }

            let w: u64 = (tiles.xpos as f32 / 16.0).round() as u64 % 2;
            let h: u64 = (tiles.ypos as f32 / 16.0).round() as u64 % 2;

            struct Positions {
                leftx: u64,
                midx: u64,
                rightx: u64,
                cleftx: u64,
                crightx: u64,
                shleftx: u64,
                shmidx: u64,
                shrightx: u64,
                svx: u64,
                topy: u64,
                midy: u64,
                bottomy: u64,
                ctopy: u64,
                cbottomy: u64,
                shtopy: u64,
                shmidy: u64,
                shbottomy: u64,
                shy: u64,
            }

            let mut pos = Positions {
                leftx: 18,
                midx: 53,
                rightx: 88,
                cleftx: 123,
                crightx: 158,
                shleftx: 18,
                shmidx: 53,
                shrightx: 88,
                svx: 123,
                topy: 1,
                midy: 36,
                bottomy: 71,
                ctopy: 106,
                cbottomy: 141,
                shtopy: 1,
                shmidy: 36,
                shbottomy: 71,
                shy: 106,
            };

            match w {
                0 => {
                    pos.leftx = 1;
                    pos.midx = 36;
                    pos.rightx = 71;
                    pos.cleftx = 106;
                    pos.crightx = 141;
                    pos.shleftx = 1;
                    pos.shmidx = 36;
                    pos.shrightx = 71;
                    pos.svx = 106;
                }
                1 => {
                    pos.leftx = 18;
                    pos.midx = 53;
                    pos.rightx = 88;
                    pos.cleftx = 123;
                    pos.crightx = 158;
                    pos.shleftx = 18;
                    pos.shmidx = 53;
                    pos.shrightx = 88;
                    pos.svx = 123;
                }
                _ => {}
            }
            match h {
                0 => {
                    pos.topy = 1;
                    pos.midy = 36;
                    pos.bottomy = 71;
                    pos.ctopy = 106;
                    pos.cbottomy = 141;
                    pos.shtopy = 1;
                    pos.shmidy = 36;
                    pos.shbottomy = 71;
                    pos.shy = 106;
                }
                1 => {
                    pos.topy = 18;
                    pos.midy = 53;
                    pos.bottomy = 88;
                    pos.ctopy = 18;
                    pos.cbottomy = 53;
                    pos.shtopy = 88;
                    pos.shmidy = 123;
                    pos.shbottomy = 158;
                    pos.shy = 123;
                }
                _ => {}
            }

            let mut tile_pos = (0, 0);
            if ml && tm && mr && bm {
                tile_pos = (pos.midx, pos.midy);
            } else {
                let cx = if !(ml && mr) {
                    if mr {
                        pos.leftx
                    } else if ml {
                        pos.rightx
                    } else {
                        141
                    }
                } else {
                    pos.midx
                };
                //
                let cy = if !(tm && bm) {
                    if bm {
                        pos.topy
                    } else if tm {
                        pos.bottomy
                    } else {
                        71
                    }
                } else {
                    pos.midy
                };
                tile_pos = (cx, cy);
            }

            TileData {
                enabled: true,
                xpos: tiles.xpos,
                ypos: tiles.ypos,
                offset_x: tile_pos.0,
                offset_y: tile_pos.1,
                tile_id: tiles.tile_id,

                tile: true,
            }
        }
    }

    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    struct Rules {
        use_ceilings: Vec<bool>, //a u8-carrying vector which essentially tracks
        //all values of transpoints and decides if there will be ceilings in this section.
        use_celings_height: Vec<u8>, //see previous field. each value given will be how much tiles a ceiling will be
        //per ceiling section.
        fortress_arena: bool, //swaps out the BORING robot master arena in place of a COOLER (and emptier) fortress boss arena.
        enemies: Vec<u16>, //u16 vector that contains all the enemy types that will be used in the level.
        //not a u8 because by the point 1.9 drops there will (likely) be more than 255 enemy ids in the game
        limit_bosstype: bool, //limits robot masters to rm levels and fort bosses (save for the darkmen) to fort levels.
        //dark man 3 and dark man 4 aren't affected by this variable due to being fort bosses that act like rms.
        limit_bosses: bool, //will likely not be in initial release. when true will allow multiple bosses in 1 level.
        bossentrance: bool, //is there a coridoor before the boss?
    }

    fn handle_weapon(mut text: String) -> String {
        //weapon system
        //will force the mega buster onto slot zero since this IS a traditional lvl
        text = format!("{}\"{}\"", format!("{}\n1k{}=", text, 0), 0);
        for i in 1..12 {
            //picks a random wpn id from versions 1.0 to 1.8.5.2, although older versions of the rng only supporterd 1.0 to 1.6.3
            let rand_num: u64 = rand::thread_rng().gen_range(1..105); //mega buster is removed from the weapon pool, unlike classic mode
            text = format!("{}\"{}\"", format!("{}\n1k{}=", text, i), rand_num);
            //unlike classic, ALL slots will be filled
        }
        text
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

    fn handle_tiling(
        text: String,
        level_length: i64,
        transpoints: Vec<i64>,
        rules: Rules,
    ) -> (String, Vec<TileData>) {
        //adds tiles
        let mut pointchecker = 0;
        let mut screeny = 0;
        let tile_id: u64 = 3;
        //rand::thread_rng().gen_range(0..1);
        //let tile_id = 3;
        let mut vecheight = Vec::new();
        #[allow(unused_assignments)]
        let arena_ceiling = rand::thread_rng().gen_range(0..4);
        for i in 0..level_length / 16 {
            match rules.fortress_arena {
                false => {
                    if i >= ((level_length - 256) / 16) {
                        for j in 0..14 {
                            if j < arena_ceiling {
                                vecheight.push(TileData {
                                    enabled: true,
                                    xpos: (i * 16) as u64,
                                    ypos: screeny + (j * 16),
                                    offset_x: 1,
                                    offset_y: 1,
                                    tile_id,

                                    tile: true,
                                });
                            }
                            if i == ((level_length - 256) / 16) || i == ((level_length) / 16) - 1 {
                                vecheight.push(TileData {
                                    enabled: true,
                                    xpos: (i * 16) as u64,
                                    ypos: screeny + (j * 16),
                                    offset_x: 1,
                                    offset_y: 1,
                                    tile_id,

                                    tile: true,
                                });
                            }
                            vecheight.push(TileData {
                                enabled: true,
                                xpos: (i * 16) as u64,
                                ypos: screeny + 224 - 16,
                                offset_x: 1,
                                offset_y: 1,
                                tile_id,

                                tile: true,
                            });
                        }
                    } else {
                        for j in 0..224 / 16 {
                            println!("{},{}", i * 16, j * 16);
                            vecheight.push(TileData {
                                enabled: true,
                                xpos: (i * 16) as u64,
                                ypos: (screeny) + (j * 16),
                                offset_x: 1,
                                offset_y: 1,
                                tile_id,

                                tile: true,
                            });

                            if pointchecker < transpoints.len()
                                && i * 16 == transpoints[pointchecker]
                            {
                                screeny += 224;
                                pointchecker += 1;
                                println!("{screeny}");
                                for y in 0..14 {
                                    for x in 0..16 {
                                        vecheight.push(TileData {
                                            enabled: true,

                                            xpos: ((x * 16) + (i * 16)) as u64,

                                            ypos: (screeny - 224) + (y * 16),
                                            offset_x: 1,
                                            offset_y: 1,
                                            tile_id,

                                            tile: true,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                true => {
                    if i >= ((level_length - 256) / 16) {
                        vecheight.push(TileData {
                            enabled: true,
                            xpos: (i * 16) as u64,
                            ypos: screeny + 224 - 16,
                            offset_x: 1,
                            offset_y: 1,
                            tile_id,

                            tile: true,
                        });
                    } else {
                        for j in 0..224 / 16 {
                            println!("{},{}", i * 16, j * 16);
                            vecheight.push(TileData {
                                enabled: true,
                                xpos: (i * 16) as u64,
                                ypos: (screeny) + (j * 16),
                                offset_x: 1,
                                offset_y: 1,
                                tile_id,

                                tile: true,
                            });

                            if pointchecker < transpoints.len()
                                && i * 16 == transpoints[pointchecker]
                            {
                                screeny += 224;
                                pointchecker += 1;
                                println!("{screeny}");
                                for y in 0..14 {
                                    for x in 0..16 {
                                        vecheight.push(TileData {
                                            enabled: true,
                                            xpos: (x * 16) + (i * 16) as u64,
                                            ypos: (screeny - 224) + (y * 16),
                                            offset_x: 1,
                                            offset_y: 1,
                                            tile_id,

                                            tile: true,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                #[allow(unreachable_patterns)]
                _ => {
                    panic!("failed to get fortress arena info");
                }
            }
        }
        print!("{}", level_length / 16);
        (text, vecheight)
    }
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    fn handle_terraform(
        vecheights: Vec<TileData>,
        rules: Rules,
        level_length: i64,
        transpoints: Vec<i64>,
    ) -> (Vec<TileData>, u64) {
        let mut prev_meta = 0;
        let mut xpos_metatile = Vec::<u64>::new();
        let mut ypos_metatile = Vec::<u64>::new();
        let mut old_mtt = MetaTile::NoTile;
        let mut mtt_array: Vec<MetaTile> = Vec::new();
        let mut thev: Vec<MetaTile> = vec![MetaTile::NoTile];
        fn check_collider(
            arr: Vec<MetaTile>,
            selfi: MetaTile,
            posx: Vec<u64>,
            posy: Vec<u64>,
            selfx: u64,
            selfy: u64,
            tiles: Vec<MetaTile>,
        ) -> MetaTile {
            let mut arr = arr;
            let mut can_notile = true;
            let mut checkx = false;
            let mut checky = false;

            for i in 0..posx.len() {
                checkx = posx[i] != 0;
                checky = posy[i] != 0;

                        match tiles[i] {
                            MetaTile::Full => arr.push(MetaTile::Full),
                            MetaTile::StairLeft => arr.push(MetaTile::StairLeft),
                            MetaTile::StairRight => arr.push(MetaTile::StairRight),
                            MetaTile::LineTop => arr.push(MetaTile::LineTop),
                            MetaTile::LineBottom => arr.push(MetaTile::LineBottom),
                            MetaTile::NoTile => arr.push(MetaTile::Full),
                            MetaTile::InvertedStairL => todo!(),
                            MetaTile::InvertedStairR => todo!(),
                            MetaTile::LineLeft => todo!(),
                            MetaTile::LineRight => todo!(),
                        }
                        can_notile = false;
                    }
                
            

            if can_notile {
                choose(vec![MetaTile::NoTile])
            } else {
                choose(arr)
            }
        }

        let mut counter = 0;
        let mut v = Vec::new();
        let mut bossentrance_y = thread_rng().gen_range(1..9) * 16;
        let mut bosschecky = 0;
        let mut terraintype: &str = "FLAT";
        let mut terraintop = thread_rng().gen_range(4..7) * 32;
        let mut screeny = 0;
        let mut terraincount = 0;

        for i in 0..vecheights.len() {
            if vecheights[i].xpos >= (level_length - 544) as u64 && vecheights[i].ypos % 224 == 0 && bosschecky == 0 {
                bosschecky = vecheights[i].ypos;
            }
            if vecheights[i].ypos % 224 == 0 && bosschecky == 0 {
                screeny = vecheights[i].ypos;
            }
            if vecheights[i].xpos < (level_length - 256) as u64 {
                if vecheights[i].xpos > (level_length - 544) as u64 {
                    let thrush = bossentrance_y + bosschecky;
                    v.push(TileData {
                        enabled: true,
                        xpos: vecheights[i].xpos,
                        ypos: thrush,
                        offset_x: vecheights[i].offset_x,
                        offset_y: vecheights[i].offset_y,
                        tile_id: vecheights[i].tile_id,

                        tile: true,
                    });
                    v.push(TileData {
                        enabled: true,
                        xpos: vecheights[i].xpos,
                        ypos: thrush + 80,
                        offset_x: vecheights[i].offset_x,
                        offset_y: vecheights[i].offset_y,
                        tile_id: vecheights[i].tile_id,

                        tile: true,
                    });
                    for f in 0..bossentrance_y / 16 {
                        v.push(TileData {
                            enabled: true,
                            xpos: vecheights[i].xpos,
                            ypos: bosschecky + f * 16,
                            offset_x: vecheights[i].offset_x,
                            offset_y: vecheights[i].offset_y,
                            tile_id: vecheights[i].tile_id,

                            tile: true,
                        });
                    }
                    for g in ((bossentrance_y / 16) + 1) + 4..14 {
                        v.push(TileData {
                            enabled: true,
                            xpos: vecheights[i].xpos,
                            ypos: bosschecky + g * 16,
                            offset_x: vecheights[i].offset_x,
                            offset_y: vecheights[i].offset_y,
                            tile_id: vecheights[i].tile_id,

                            tile: true,
                        });
                    }
                } else if bosschecky != 0 && vecheights[i].ypos < (bossentrance_y + bosschecky + 16)
                    || vecheights[i].ypos > (bossentrance_y + bosschecky) + 64
                {
                    if vecheights[i].xpos % 32 == 0 && vecheights[i].ypos % 32 == 0 {
                        let mut tempx = 0;
                        let mut tempy = 0;
                        match terraintype {
                            "FLAT" => {
                                terraincount += 1;
                                if vecheights[i].ypos < terraintop+screeny {
                                    tempx = u64::MAX;
                                }
                                else {
                                    tempx = vecheights[i].xpos;
                                }
                                tempy = vecheights[i].ypos;
                                if terraincount >= rand::thread_rng().gen_range(1..5) {
                                    terraintype = choose(vec!["FLAT", "PITS", "PITS"]);

                                    terraintop = thread_rng().gen_range(4..7) * 32_u64;
                                    terraincount = 0;
                                }
                            }
                            "PITS" => {
                                tempy = 0;
                                tempx = u64::MAX;

                                terraintype = "FLAT";
                                terraintop = thread_rng().gen_range(4..7) * 32_u64;
                                terraincount = 0;
                            }
                            _ => {
                                tempx = vecheights[i].xpos;
                                tempx = vecheights[i].ypos;
                            }
                        }
                        if tempx != u64::MAX {
                            let mtt = choose(thev.clone());
                            let mut o = old_mtt;

                            xpos_metatile.push(tempx);
                            ypos_metatile.push(tempy);

                            mtt_array.push(mtt);
                            o = check_collider(
                                mtt_array.clone(),
                                o,
                                xpos_metatile.clone(),
                                ypos_metatile.clone(),
                                tempx,
                                tempy,
                                mtt_array.clone(),
                            );

                            let meta = MetaTile::from(o, vecheights[i].tile_id, tempx, tempy);
                            for fa in 0..meta.len() {
                                v.push(meta[fa]);
                            }
                            println!("{:?}", meta.clone());
                        }
                    }
                } else if ((bosschecky == 0 && vecheights[i].xpos < (level_length - 544) as u64) || (bosschecky != 0 && vecheights[i].xpos < (level_length - 544) as u64)
                        && vecheights[i].ypos > (bossentrance_y + bosschecky + 16)
                        && vecheights[i].ypos < (bossentrance_y + bosschecky) + 64) && vecheights[i].xpos % 32 == 0 && vecheights[i].ypos % 32 == 0 {
                    let mut tempx = 0;
                    let mut tempy = 0;
                    match terraintype {
                        "FLAT" => {
                            
                            terraincount += 1;
                            tempx = vecheights[i].xpos;

                            tempy = vecheights[i].ypos;
                            if terraincount >= rand::thread_rng().gen_range(1..5) {
                                terraintype = choose(vec!["FLAT", "PITS", "PITS"]);

                                terraintop = thread_rng().gen_range(4..7) * 32_u64;
                                terraincount = 0;
                            }
                        }
                        "PITS" => {
                            tempy = 0;
                            tempx = u64::MAX;

                            terraintype = "FLAT";
                            terraintop = thread_rng().gen_range(4..7) * 32_u64;
                            terraincount = 0;
                        }
                        _ => {
                            tempx = vecheights[i].xpos;
                            tempx = vecheights[i].ypos;
                        }
                    }
                    if tempx != u64::MAX {
                        let mtt = choose(thev.clone());
                        let mut o = old_mtt;

                        xpos_metatile.push(tempx);
                        ypos_metatile.push(tempy);

                        mtt_array.push(mtt);
                        o = check_collider(
                            mtt_array.clone(),
                            o,
                            xpos_metatile.clone(),
                            ypos_metatile.clone(),
                            tempx,
                            tempy,
                            mtt_array.clone(),
                        );

                        let meta = MetaTile::from(o, vecheights[i].tile_id, tempx, tempy);
                        for fa in 0..meta.len() {
                            v.push(meta[fa]);
                        }
                        println!("{:?}", meta.clone());
                    }
                }
            } else if vecheights[i].xpos >= (level_length - 256) as u64 {
                if vecheights[i].ypos % 224 == 0 && bosschecky == 0 {
                    bosschecky = vecheights[i].ypos;
                }
                if ((vecheights[i].ypos < (bosschecky + bossentrance_y) + 16)
                    && vecheights[i].xpos <= (level_length - 256) as u64
                    || vecheights[i].ypos > ((bosschecky + bossentrance_y) + 64)
                        && vecheights[i].xpos <= (level_length - 256) as u64)
                    || vecheights[i].xpos != (level_length - 256) as u64
                {
                    v.push(TileData {
                        enabled: true,
                        xpos: vecheights[i].xpos,
                        ypos: vecheights[i].ypos,
                        offset_x: vecheights[i].offset_x,
                        offset_y: vecheights[i].offset_y,
                        tile_id: vecheights[i].tile_id,

                        tile: true,
                    });
                }
            }
            /*
            v.push(vecheights[counter].clone());
            println!("{:?}", vecheights[counter]);

            /*TileData {
                enabled: true,
                xpos: 0,
                ypos: 0,
                offset_x: 1,
                offset_y: 1,
                tile_id: 0,
                extra_e: Some(format!("{}", 0)),
                tile: true,
            }
            */
            counter += 1;
            */
        }
        println!("{}", bosschecky);
        (v, bosschecky + bossentrance_y)
    }

    fn handle_boss(
        contents: String,
        bossid: u64,
        level_length: i64,
        screeny: u64,
        rules: Rules,
    ) -> String {
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

        match rules.fortress_arena {
            true => {
                bossx = level_length - 48;
                bossy = (screeny + 224) - 32;
            }
            false => {
                bossx = level_length - 48;
                bossy = (screeny + 224) - 32;
            }
        }
        let mut text = format!(
            "{}a{},{}=\"1\"\nb{},{}=\"1\"\nc{},{}=\"1\"\nd{},{}=\"8\"\ne{},{}=\"{}\"\n",
            contents, bossx, bossy, bossx, bossy, bossx, bossy, bossx, bossy, bossx, bossy, bossid
        );
        text = format!("{}1xc0=\"{}\"\n1yc0=\"{}\"\n1ga0=\"1\"\n1g0=\"1\"\n1ha0=\"0\"\n1h0=\"1\"\n1i0=\"0\"\n1j0=\"0\"\n1n0=\"{}\"\n1o0=\"0\"\n",text,bossx,bossy, mus);
        println!("boss at {},{}. id is {}", bossx, bossy, bossid);
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

    pub fn file_write(batch: bool) {
        let mut batchloop = 10;
        if !batch {
            batchloop = 1;
        };
        for counts in 0..batchloop {
            let bgcount = rand::thread_rng().gen_range(0..856);

            let length: i64 = (rand::thread_rng().gen_range(9..23)) * 256;
            //screen trans
            let mut transpoints = Vec::new();

            for c in 0..length / 256 {
                let transition = rand::thread_rng().gen_bool(1.0 / 4.0);

                if transition && c * 256 < length - 768 {
                    transpoints.push(c * 256);
                }
            }

            let mut pointchecker = 0;
            let mut screeny = 0;

            //naming
            let fortress = rand::thread_rng().gen_bool(1.0 / 4.5);

            fn read_lines(filename: &str) -> Vec<String> {
                read_to_string(filename)
                    .unwrap() // panic on possible file-reading errors
                    .lines() // split the string into an iterator of string slices
                    .map(String::from) // make each slice into a string
                    .collect() // gather them together into a vector
            }

            let names = read_lines("names.txt");
            let mut name = String::new();

            if fortress {
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

            let mut rule = Rules {
                use_ceilings: Vec::new(),
                use_celings_height: Vec::new(),
                enemies: Vec::new(),
                fortress_arena: false,
                limit_bosses: true,
                limit_bosstype: true,
                bossentrance: fortress,
            };
            for t in 0..transpoints.len() {
                rule.use_ceilings.push(thread_rng().gen_bool(1.0 / 2.0));
                rule.use_celings_height.push(thread_rng().gen_range(1..5));
            }
            rule.limit_bosses = true;
            rule.fortress_arena = fortress;
            rule.limit_bosstype = true;
            //init
            let mut contents = String::from("[Level]");
            //weapons
            let binding = handle_weapon(contents.clone());
            contents = binding;
            //general things
            let mugshot = rand::thread_rng().gen_range(1..41); //boss mugshot id
            contents = format!("{}\n0v=\"1.9.0\"\n1a=\"{}\"\n4a=\"MMMRNG\"\n4b=\"{}\"\n0a=\"000000\"\n1p=\"0\"\n1q=\"{}\"\n1r=\"0\"\n1s=\"4480\"\n1bc=\"3\"\n1f=\"{}\"\n1e=\"{}\"\n", contents,name,rand::thread_rng().gen_range(0..161),length,mugshot,rand::thread_rng().gen_range(0..51)); //adds general level info
                                                                                                                                                                                                                                                                                              //musica
            let binding = handle_music(contents.clone());
            contents = binding;
            //player abilities
            let binding = handle_abilities(contents.clone());
            contents = binding;

            //activate sections and add backgrounds
            for i in -1..length / 256 {
                if pointchecker < transpoints.len() && i - 1 == transpoints[pointchecker] / 256 {
                    if i != -1 {
                        screeny += 224;
                        pointchecker += 1;
                        contents =
                            format!("{}2a{},{}=\"1\"\n", contents, (i - 1) * 256, screeny);
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

                        contents =
                            format!("{}2d{},{}=\"{}\"\n", contents, i * 256, screeny, bgcount);
                        println!("{screeny}");
                    }
                }
                if i != -1 {
                    contents = format!("{}2a{},{}=\"1\"\n", contents, i * 256, screeny);
                    //add bg
                    contents = format!("{}2d{},{}=\"{}\"\n", contents, i * 256, screeny, bgcount);
                    //lock screen on boss
                    println!("section at {},{} activated.", i * 256, screeny);
                    if i == (length / 256) - 1 {
                        pointchecker += 1;
                        contents = format!("{}2b{},{}=\"0\"\n", contents, (i) * 256, screeny);
                    }
                    if rule.bossentrance && i == (length / 256) - 2 {
                        pointchecker += 1;
                        contents = format!("{}2b{},{}=\"0\"\n", contents, (i) * 256, screeny);
                    }
                }
            }
            contents = format!("{}2b{},{}=\"0\"\n", contents, length - 512, screeny);
            //TILING!!!!!!!!!!!
            let binding =
                handle_tiling(contents.clone(), length, transpoints.clone(), rule.clone());
            contents = binding.0;
            let mut vecheights: Vec<TileData> = binding.1;
            //terraforming

            let binding = handle_terraform(vecheights, rule.clone(), length, transpoints.clone());
            vecheights = binding.0;
            contents = format!(
                "{}a{},{}=\"1\"\nb{},{}=\"1\"\nc{},{}=\"1\"\nd{},{}=\"8\"\ne{},{}=\"33\"\na{},{}=\"1\"\nb{},{}=\"1\"\nc{},{}=\"1\"\nd{},{}=\"8\"\ne{},{}=\"33\"\n",
                contents,

                length - 512,
                binding.1+64,
                length - 512,
                binding.1+64,
                length - 512,
                binding.1+64,
                length - 512,
                binding.1+64,
                length - 512,
                binding.1+64,

                length - 256,
                binding.1+64,
                length - 256,
                binding.1+64,
                length - 256,
                binding.1+64,
                length - 256,
                binding.1+64,
                length - 256,
                binding.1+64,

            ); //boss dor

            //auto tiling
            /*
            for i in 0..vecheights.len() {
                vecheights[i] = TileData::autotile_prep(&vecheights[i].clone(), &vecheights);
            }
            */
            for i in 0..vecheights.len() {
                contents = format!(
                    "{}a{},{}=\"{}\"\ne{},{}=\"{}\"\ni{},{}=\"1\"\nj{},{}=\"{}\"\nk{},{}=\"{}\"\n",
                    contents,
                    vecheights[i].xpos,
                    vecheights[i].ypos,
                    vecheights[i].enabled.as_int(),
                    vecheights[i].xpos,
                    vecheights[i].ypos,
                    vecheights[i].tile_id,
                    vecheights[i].xpos,
                    vecheights[i].ypos,
                    vecheights[i].xpos,
                    vecheights[i].ypos,
                    vecheights[i].offset_x,
                    vecheights[i].xpos,
                    vecheights[i].ypos,
                    vecheights[i].offset_y
                );
            }
            let objpoints = transpoints.clone();

            //oh and object placements
            //let binding = handle_megaman(contents.clone(),vecheights.clone());
            //contents = binding.0;
            //let binding = handle_objs(contents.clone(),vecheights.clone(),length,binding.1,binding.2,objpoints);
            //contents = binding;

            //handle boss placement
            let mut bossid;
            loop {
                bossid = rand::thread_rng().gen_range(1..70);
                if bossid != 9 && bossid != 0 && bossid != 1 && bossid != 36 && bossid <= 55 {
                    // disables fortress bosses plus supressors and stuff for testing purposes. also disables gemini man (HOPEFULLY)
                    break;
                } else {
                    continue;
                }
            }
            let binding = handle_boss(
                contents.clone(),
                bossid,
                length,
                screeny,
                rule,
            );
            contents = binding;

            //write all data to the mmlv file.
            fs::write("level.mmlv", contents.clone()).expect("failed to write mmlv");
            if batch {
                fs::rename("level.mmlv", format!("level{}.mmlv", counts + 1));
            }
        }
    }
}
