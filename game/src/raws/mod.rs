rltk::embedded_resource!(
    GAME_CONFIG,
    "/home/ganiparrott/src/projects/rust_book/roguelike/raws/game.txt"
);
rltk::embedded_resource!(
    MODEL_CONFIG,
    "/home/ganiparrott/src/projects/rust_book/roguelike/raws/model.txt"
);

pub fn load_raws() {
    rltk::link_resource!(
        GAME_CONFIG,
        "/home/ganiparrott/src/projects/rust_book/roguelike/raws/game.txt"
    );
    rltk::link_resource!(
        MODEL_CONFIG,
        "/home/ganiparrott/src/projects/rust_book/roguelike/raws/model.txt"
    );
}
