var config = {
    type: Phaser.AUTO,
    width: %WIDTH%,
    height: %HEIGHT%,
    scene: {
        preload: preload,
        create: create,
        update: update
    }
};

var game = new Phaser.Game(config);

