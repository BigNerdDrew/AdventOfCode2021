fn main() {
    let original_fish = vec![3,5,1,5,3,2,1,3,4,2,5,1,3,3,2,5,1,3,1,5,5,1,1,1,2,4,1,4,5,2,1,2,4,3,1,2,3,4,3,4,4,5,1,1,1,1,5,5,3,4,4,4,5,3,4,1,4,3,3,2,1,1,3,3,3,2,1,3,5,2,3,4,2,5,4,5,4,4,2,2,3,3,3,3,5,4,2,3,1,2,1,1,2,2,5,1,1,4,1,5,3,2,1,4,1,5,1,4,5,2,1,1,1,4,5,4,2,4,5,4,2,4,4,1,1,2,2,1,1,2,3,3,2,5,2,1,1,2,1,1,1,3,2,3,1,5,4,5,3,3,2,1,1,1,3,5,1,1,4,4,5,4,3,3,3,3,2,4,5,2,1,1,1,4,2,4,2,2,5,5,5,4,1,1,5,1,5,2,1,3,3,2,5,2,1,2,4,3,3,1,5,4,1,1,1,4,2,5,5,4,4,3,4,3,1,5,5,2,5,4,2,3,4,1,1,4,4,3,4,1,3,4,1,1,4,3,2,2,5,3,1,4,4,4,1,3,4,3,1,5,3,3,5,5,4,4,1,2,4,2,2,3,1,1,4,5,3,1,1,1,1,3,5,4,1,1,2,1,1,2,1,2,3,1,1,3,2,2,5,5,1,5,5,1,4,4,3,5,4,4];
    
    let original_fish_count = original_fish.len();
    let mut fish_spawn_calendar = vec![0; 256+9];
    
    for fish in original_fish {
        fish_spawn_calendar[fish] += 1;
    }
    
    for day in 0..256 {
        
        let mut fish_spawning_today = 0;
        
        let mut day_rewind = day;
        
        loop {
            fish_spawning_today += fish_spawn_calendar[day_rewind];
            
            if day_rewind < 7 { break; }
            day_rewind -= 7;
        }
        
        let next_fish_spawn_day = day + 9;
        fish_spawn_calendar[next_fish_spawn_day] += fish_spawning_today;
        
        println!("day {} complete", day);
    }
    
    
    let fish_count: i64 = fish_spawn_calendar.iter().sum::<i64>();
    
    println!("fish after 256 days: {}", fish_count);
}