from engine import RenderEngine
from scene import load_scene

def test_engine():
    scene = load_scene("examples/scene1.yaml")
    engine = RenderEngine()
    image = engine.render(scene)
