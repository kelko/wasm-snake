import init, {Display, World, GameState, WorldStatus} from "snake_game";

const A_SECOND = 1000;
let frames_per_second = 6;

const canvasElement = document.getElementById("snake-canvas") as HTMLCanvasElement;
const canvas = canvasElement.getContext("2d")

await init();
const world = World.new();
const dimensions = world.get_dimension()

const CELL_SIZE = 12;
const renderer = Display.new(canvas, CELL_SIZE);
canvasElement.height = dimensions.height * CELL_SIZE;
canvasElement.width = dimensions.width * CELL_SIZE;

function paint(status: WorldStatus) {
    renderer.render(dimensions, status);
}

function adjustDifficulty(status: WorldStatus) {
    // why these numbers? Trial & Error
    frames_per_second = 6 * (status.level * 0.25) + 0.75;
}

function update() {
    setTimeout(() => {
        const newStatus = world.next_frame();
        adjustDifficulty(newStatus)
        paint(newStatus);

        if (newStatus.state === GameState.Play || newStatus.state === GameState.Pause) {
            requestAnimationFrame(update)
        }
    }, A_SECOND / frames_per_second)
}

window.addEventListener("keydown", evt => {
    switch (evt.code) {
        case "ArrowRight":
            world.face_right();
            break;

        case "ArrowLeft":
            world.face_left();
            break;

        case "ArrowUp":
            world.face_up();
            break;

        case "ArrowDown":
            world.face_down();
            break;

        case "KeyP":
            world.toggle_pause();
            break;
    }
})

paint(world.get_status())
update();
