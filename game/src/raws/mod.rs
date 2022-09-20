rltk::embedded_resource!(
    GAME_CONFIG,
    "raws/game.txt"
);
rltk::embedded_resource!(
    MODEL_CONFIG,
    "raws/model.txt"
);

pub fn load_raws() {
    rltk::link_resource!(
        GAME_CONFIG,
        "raws/game.txt"
    );
    rltk::link_resource!(
        MODEL_CONFIG,
        "raws/model.txt"
    );
}
