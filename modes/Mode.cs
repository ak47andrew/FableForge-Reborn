namespace Vtt.Modes;

public abstract class Mode
{
    public abstract void Draw();
    public abstract void Update(float deltaTime);
}