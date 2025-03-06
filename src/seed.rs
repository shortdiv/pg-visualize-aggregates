use crate::models::Climbs;

pub fn load_climb_data() -> Vec<Climbs> {
    vec![
        Climbs {
            id: 219,
            route_name: "Unknown 3".to_string(),
            parent_sector: "Left Of Waterfall".to_string(),
            route_id: 116216253,
            sector_id: 116214477,
            type_string: "sport".to_string(),
            fa: "Thurmond".to_string(),
            yds: "5.12+".to_string(),
            vermin: "".to_string(),
            nopm_yds: "5.12c/d".to_string(),
            nopm_vermin: "".to_string(),
            yds_rank: "125.0".to_string(),
            vermin_rank: "".to_string(),
            safety: "".to_string(),
            parent_loc: "[-90.26339, 38.35438]".to_string(),
            description: "['Climb up a bludge and through the overhung face']".to_string(),
            location: "['Sport route to the right of a small cave feature.']".to_string(),
            protection: "['4 bolts + anchor']".to_string(),
            corrected_users_ratings: "[('1685717541b3763005c89bfc5890169a44447b4c', 1.0)]"
                .to_string(),
        },
        Climbs {
            id: 220,
            route_name: "Unknown 2".to_string(),
            parent_sector: "Left Of Waterfall".to_string(),
            route_id: 116216253,
            sector_id: 116214477,
            type_string: "sport".to_string(),
            fa: "Thurmond".to_string(),
            yds: "5.12+".to_string(),
            vermin: "".to_string(),
            nopm_yds: "5.12c/d".to_string(),
            nopm_vermin: "".to_string(),
            yds_rank: "125.0".to_string(),
            vermin_rank: "".to_string(),
            safety: "".to_string(),
            parent_loc: "[-90.26339, 38.35438]".to_string(),
            description: "['Climb up a bludge and through the overhung face']".to_string(),
            location: "['Sport route.']".to_string(),
            protection: "['5 bolts + anchor']".to_string(),
            corrected_users_ratings: "[('1685717541b3763005c89bfc5890169a44447b4c', 1.0)]"
                .to_string(),
        },
        Climbs {
            id: 221,
            route_name: "Unknown 2".to_string(),
            parent_sector: "Left Of Waterfall".to_string(),
            route_id: 116216253,
            sector_id: 116214477,
            type_string: "sport".to_string(),
            fa: "Thurmond".to_string(),
            yds: "5.12+".to_string(),
            vermin: "".to_string(),
            nopm_yds: "5.12b/c".to_string(),
            nopm_vermin: "".to_string(),
            yds_rank: "123.0".to_string(),
            vermin_rank: "".to_string(),
            safety: "".to_string(),
            parent_loc: "[-90.26339, 38.35438]".to_string(),
            description: "['']".to_string(),
            location: "['']".to_string(),
            protection: "['4 bolts + anchor']".to_string(),
            corrected_users_ratings: "[('1685717541b3763005c89bfc5890169a44447b4c', 1.0)]"
                .to_string(),
        },
    ]
}

// INSERT INTO climbs(id,route_name,parent_sector,route_id,sector_id,type_string,fa,yds,vermin,nopm_yds,nopm_vermin,yds_rank,vermin_rank,safety,parent_loc,description,location,protection,corrected_users_ratings) VALUES (219,'Unknown 3','Left Of Waterfall',116216253,116214477,'sport','Thurmond','5.12+',null,'5.12c/d',null,'125.0',null,null,'[-90.26339, 38.35438]','["Climb up a bludge and through the overhung face"]','["Sport route to the right of a small cave feature."]','["4 bolts + anchor"]',null),(220,'Unknown 2','Left Of Waterfall',116216253,116214477,'sport','Thurmond','5.12+',null,'5.12c/d',null,'125.0',null,null,'[-90.26339, 38.35438]','["Climb up a bludge and through the overhung face"]','["Sport route."]','["5 bolts + anchor"]',null),(221,'Unknown 2','Left Of Waterfall',116216253,116214477,'sport','Thurmond','5.12+',null,'5.12b/c',null,'123.0',null,null,'[-90.26339, 38.35438]','[""]','[""]','["4 bolts + anchor"]',null);
