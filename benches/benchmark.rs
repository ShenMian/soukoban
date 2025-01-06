use std::{fs, path::Path, str::FromStr};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nalgebra::Vector2;
use soukoban::{
    deadlock::*,
    path_finding::box_move_waypoints,
    solver::{Solver, Strategy},
    Level, Map,
};

pub fn load_level_from_file<P: AsRef<Path>>(path: P, id: usize) -> Level {
    debug_assert!(id >= 1);
    Level::load_nth_from_str(&fs::read_to_string(path).unwrap(), id).unwrap()
}

// Title: World Cup 2014 (MF8 61st Sokoban Competition, Extra)
// Author: laizhufu
const WORLDCUP2014: &str = r#"
    -------#########-------
    -----##---------##-----
    ---##---#####--#--##---
    --#---##------#--#--#--
    --####---##--#--#--##--
    -#-----##---#--#--#--#-
    -######----#--#--#--##-
    #-------##---#--#--#--#
    ########----#--#--#--##
    #-------.*#---#--#--#-#
    #-#-#-#-*-*-$*--*--**-#
    #-#-#-#---*-*-*-*-*-*-#
    #--#-#-#-*--*-*-*-*@*-#
    ##-#-#-#-*-**-*-*-$***#
    -#--#-#-#-*-*-*-*---*#-
    -##-#-#-#----*--.-#-##-
    --#--#-#-#-#-#-#-#--#--
    --##-#-#-#-#-#-#-#-##--
    ---##-#-#-#-#-#-#-##---
    ------#-#-#-#-#-#------
    -----#-#-#-#-#-#-#-----
    -----#-#-#-#-#-#-#-----
    ------#-#-#-#-#-#------
    ------#-#-#-#-#-#------
    ------#--#-#-#-#-#-----
    ------##-#-#-#-#-#-----
    ------##--#-#-#-#------
    ------###-#-#-#-#------
    -----####--#-#-#-#-----
    -----#####-#-#-#-#-----
    -----#####--#-#-#-#----
    ----#######-#-#-#-#----
    ----#######--#-#-#-#---
    -----#######---#-#-#---
    ---#--########----#-#--
    -#--#--##########----#-
    --#--#--#############--
    ---#--#-###########----
    -------#--######-------
"#;

pub const PATH: &str = r#"
    Title: 一箭十万 (102547步)
    Author: 20603
    ##################################################
    #   #@    #   ##  #   ##  #   ##  #   ##  #   #  #
    # # #     #       #       #       #       #      #
    # #  # ## ## ###  ## ###  ## ###  ## ###  ## ##  #
    #  # #$#      ## ###  ## ###  ## ###  ## ###  #  #
    ## #   #   #  #       #       #       #       #  #
    #  #########  #   ##  #   ##  #   ##  #   ##  #  #
    # #  #   #  ######  ######  ######  ######  ### ##
    # #      #       #       #       #       #       #
    # #  ## ##  ###  #  ###  #  ###  #  ###  #  ## # #
    # ## #  ### ##  ### ##  ### ##  ### ##  ### ##   #
    #  # #       #       #       #       #        ####
    ## # #  ##   #  ##   #  ##   #  ##   #  ###   #  #
    #  # ###  ######  ######  ######  ######  #####  #
    # #       #       #       #       #       #      #
    # #   ##  #  ###  #  ###  #  ###  #  ###  #  ## ##
    #  ##### ###  ## ###  ## ###  ## ###  ## ###  # ##
    ## #  #       #       #       #       #       # ##
    #  #  #   ##  #   ##  #   ##  #   ##  #   ##  # ##
    # ##  ######################################### ##
    #   #                                            #
    ### # ########################################   #
    #   # #   ##  #   ##  #   ##  #   ##  #   ##  ####
    # ### #       #       #       #       #       #  #
    #  ## ## ###  ## ###  ## ###  ## ###  ## ###  #  #
    ## #      ## ###  ## ###  ## ###  ## ###  ## ##  #
    #  #   #  #       #       #       #       #     ##
    # ######  #   ##  #   ##  #   ##  #   ##  #   # ##
    #     ######################################### ##
    ##### #  ##   #  ##   #  ##   #  ##   #  ##   # ##
    #     #       #       #       #       #       # ##
    # #####  ### ##  ### ##  ### ##  ### ##  ### ## ##
    #  #  ## ##  ### ##  ### ##  ### ##  ### ##  ## ##
    ## #      #       #       #       #       #      #
    #  #  #   #  ##   #  ##   #  ##   #  ##   #  #   #
    # ### ############################################
    #  ## ##  #   ##  #   ##  #   ##  #   ##  #   #  #
    ## #      #       #       #       #       #      #
    #  #   #  ## ###  ## ###  ## ###  ## ###  ## ##  #
    #  ##### ###  ## ###  ## ###  ## ###  ## ###  #  #
    # #   #       #       #       #       #       #  #
    #   # #   ##  #   ##  #   ##  #   ##  #   ##  #  #
    ##### #########################################  #
    #   # #  ##   #  ##   #  ##   #  ##   #  ##   #  #
    #     #       #       #       #       #       #  #
    ## # .#  ### ##  ### ##  ### ##  ### ##  ### ##  #
    #  ##### ##  ### ##  ### ##  ### ##  ### ##  ## ##
    #         #       #       #       #       #      #
    #  ####   #  ##   #  ##   #  ##   #  ##   #  #   #
    ##################################################
"#;

fn tunnel_benchmark(c: &mut Criterion) {
    let level = Level::from_str(PATH).unwrap();
    let solver = Solver::new(level.map().clone(), Strategy::Fast);
    solver.lower_bounds();
    c.bench_function("calculate tunnels", |b| {
        let solver = solver.clone();
        b.iter(|| {
            black_box(solver.tunnels());
        })
    });
}

fn solver_benchmark(c: &mut Criterion) {
    let mut bench_solve = |level: Level| {
        c.bench_function(
            &format!("solve level '{}' using A*", level.metadata()["title"]),
            |b| {
                let solver = black_box(Solver::new(level.map().clone(), Strategy::Fast));
                b.iter(|| {
                    solver.a_star_search().unwrap();
                })
            },
        );
        c.bench_function(
            &format!("solve level '{}' using IDA*", level.metadata()["title"]),
            |b| {
                let solver = black_box(Solver::new(level.map().clone(), Strategy::Fast));
                b.iter(|| {
                    solver.ida_star_search().unwrap();
                })
            },
        );
    };

    let level = Level::from_str(PATH).unwrap();
    bench_solve(level);

    let level = load_level_from_file("assets/BoxWorld_100.xsb", 3);
    bench_solve(level);
}

fn deadlock_benchmark(c: &mut Criterion) {
    let map = Map::from_str(WORLDCUP2014).unwrap();
    c.bench_function("calculate unused floors", |b| {
        b.iter(|| black_box(calculate_unused_floors(black_box(map.clone()))))
    });
    c.bench_function("calculate dead positions", |b| {
        b.iter(|| black_box(calculate_static_deadlocks(black_box(&map))))
    });
}

fn path_finding_benchmark(c: &mut Criterion) {
    let level = Level::from_str(PATH).unwrap();
    c.bench_function("box move waypoints", |b| {
        b.iter(|| box_move_waypoints(black_box(level.map()), Vector2::new(6, 4)))
    });
}

fn map_benchmark(c: &mut Criterion) {
    let map = Map::from_str(WORLDCUP2014).unwrap();
    c.bench_function("normalize map", |b| {
        b.iter(|| black_box(map.clone()).normalize())
    });
}

fn level_benchmark(c: &mut Criterion) {
    let mut buf = String::new();
    for entry in fs::read_dir("assets/").unwrap() {
        let path = entry.unwrap().path();
        buf += &(fs::read_to_string(path).unwrap() + "\n\n");
    }

    c.bench_function("load levels from str", |b| {
        b.iter(|| black_box(Level::load_from_str(black_box(&buf)).count()))
    });
    c.bench_function("load the nth level from str", |b| {
        b.iter(|| black_box(Level::load_nth_from_str(black_box(&buf), black_box(3371)).unwrap()))
    });
}

criterion_group!(
    benches,
    tunnel_benchmark,
    solver_benchmark,
    deadlock_benchmark,
    path_finding_benchmark,
    map_benchmark,
    level_benchmark
);
criterion_main!(benches);
