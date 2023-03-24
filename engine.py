from color import Color
from image import Image
from point import Point
from ray import Ray
from scene import Scene
from sphere import Sphere
from utils import st_time
from vector import Vector3


class RenderEngine:
    """Renders 3D object into 2D using raytracing"""
    MAX_DEPTH = 5
    MAX_DISTANCE = 10.0
    MIN_DISPLACE = 0.0001

    def __init__(self):
        self.specular_k = 50

    @st_time
    def render(self, scene: Scene) -> Image:
        """Render scene to image"""
        width, height = scene.width, scene.height
        aspect_ratio = width / height
        x0, x1 = -1.0, 1.0
        y0, y1 = x0 / aspect_ratio, x1 / aspect_ratio
        dx, dy = (x1 - x0) / (width - 1), (y1 - y0) / (height - 1)

        pixels = Image(width, height)

        for iy in range(height):
            y = y0 + iy * dy
            for ix in range(width):
                x = x0 + ix * dx
                ray = Ray(scene.camera, Point(x, y, 0) - scene.camera)
                pixels.set_pixel(ix, iy, self.ray_trace(ray, scene))
            # progress
            print(f"{iy/height:3.1%}", end="\r")
        return pixels

    def ray_trace(self, ray: Ray, scene: Scene, depth:int = 0) -> Color:
        """Cast ray and calculate color"""
        color = Color.BLACK
        distance_hit, object_hit = self.find_nearest(ray, scene)
        if object_hit is None or distance_hit > self.MAX_DISTANCE:
            return color
        hit_position = ray.origin + ray.direction * distance_hit
        hit_normal = object_hit.normal(hit_position)
        color += self.color_at(object_hit, hit_position, hit_normal, scene)

        # trace bounce ray
        if depth < self.MAX_DEPTH:
            new_ray = Ray(
                hit_position + hit_normal * self.MIN_DISPLACE,
                ray.direction - 2 * ray.direction.dot(hit_normal) * hit_normal
            )
            color += self.ray_trace(new_ray, scene, depth+1) * object_hit.material.reflection

        return color

    def find_nearest(self, ray: Ray, scene: Scene) -> Sphere:
        """Find the closest object hit by ray, return distance and object"""
        dist_min, objet_hit = None, None
        for obj in scene.objects:
            dist = obj.intersects(ray)
            if dist is not None and (objet_hit is None or dist < dist_min):
                dist_min, objet_hit = dist, obj
        return dist_min, objet_hit

    def color_at(self, object_hit: Sphere, hit_position: Point, hit_normal: Vector3, scene: Scene) -> Color:
        """Get color at hit point from each light, by combing ambient, diffuse and specular color"""
        material = object_hit.material
        obj_color = material.color_at(hit_position)
        to_cam = scene.camera - hit_position
        color = material.ambient * Color.WHITE

        for light in scene.lights:
            to_light = Ray(hit_position, light.position - hit_position)
            # Diffuse shading (Lambert)
            color += obj_color * material.diffuse * max(hit_normal.dot(to_light.direction), 0)
            # Specular shading (Blinn-Phong)
            half_vector = (to_light.direction + to_cam).normalize()
            color += light.color * material.specular * max(hit_normal.dot(half_vector), 0) ** self.specular_k
        return color