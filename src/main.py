import importlib

from engine import RenderEngine
import argparse
from multiprocessing import cpu_count


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("scene", help="Scene name")
    parser.add_argument("-p", "--processes", action="store", type=int, dest="processes", default=0,
                        help="Number of processes used by rendering (0=auto)")
    args = parser.parse_args()
    if args.processes == 0:
        process_count = cpu_count()
    else:
        process_count = args.processes
    # print("process_count", process_count)
    scene = importlib.import_module("examples." + args.scene)

    engine = RenderEngine()
    image = engine.render(scene.scene)

    file_path = "renders/" + scene.scene_name + ".ppm"
    image.save(file_path)
    # show(file_path)


def show(image_path):
    import cv2
    img = cv2.imread(image_path)
    cv2.imshow('img', img)
    cv2.waitKey(0)
    cv2.destroyAllWindows()


if __name__ == "__main__":
    main()
