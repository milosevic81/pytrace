import json


class Scene:
    """Scene has all information needed to render"""

    def __init__(self, camera, objects, lights, width, height):
        self.camera = camera
        self.objects = objects
        self.lights = lights
        self.width = width
        self.height = height

    @classmethod
    def from_json_file(cls, file_path):
        with open(file_path, "r") as f:
            data = json.load(f)
            return Scene(**data)

    def save(self, file_path):
        with open(file_path, "w") as f:
            json.dump(self.__dict__, f, default=lambda o: o.__dict__ , indent=4)
