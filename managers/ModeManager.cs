using Vtt.Modes;

namespace Vtt.Managers;

public class ModeManager
{
    private static ModeManager? instance;

    private Mode? mode;

    private ModeManager()
    {
        mode = null;
    }

    public static ModeManager getInstance()
    {
        if (instance == null)
        {
            instance = new ModeManager();
        }
        return instance;
    }

    public Mode getCurrentMode()
    {
        if (mode == null)
        {
            throw new InvalidOperationException("Mode is not set!");
        }
        return mode;
    }

    public void setMode(Mode mode)
    {
        this.mode = mode;
    }
}
