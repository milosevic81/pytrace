from engine import RenderEngine
import argparse
from scene_parser import load_scene


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("scene", help="Scene name")
    parser.add_argument(
        "-p",
        "--processes",
        action="store",
        type=int,
        dest="processes",
        default=0,
        help="Number of processes used by rendering (0=auto)",
    )
    args = parser.parse_args()
    # Use default number of processes or specified number
    # if args.processes == 0:
    #     process_count = cpu_count()
    # else:
    #     process_count = args.processes
    # print("process_count", process_count)
    # scene = importlib.import_module("examples." + args.scene)
    scene = load_scene(args.scene)

    engine = RenderEngine()
    image = engine.render(scene)

    # image.save_as_ppm("renders/" + scene.scene_name + ".ppm")
    image.save_as_png("renders/" + scene.scene_name + ".png")


if __name__ == "__main__":
    main()
