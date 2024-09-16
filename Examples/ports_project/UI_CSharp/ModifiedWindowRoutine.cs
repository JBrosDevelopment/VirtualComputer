using System;
using System.Collections.Generic;
using SFML.Graphics;
using SFML.System;
using SFML.Window;

namespace SFML_Game_Loop;

public class ModifiedWindowRoutine
{
    public Color BackgroundColor = new Color(125, 125, 125);

    public List<Drawable> Elements = new List<Drawable>();

    public const string DefaultFontPath = ".\\Fonts\\Roboto\\Roboto-Regular.ttf";

    public RenderWindow Window { get; private set; }

    public Vector2f Center => new Vector2f(Window.Size.X / 2, Window.Size.Y / 2);

    public ModifiedWindowRoutine()
    {
    }

    public ModifiedWindowRoutine(VideoMode videoMode, string title, Styles style = Styles.Default, Color? color = null)
    {
        RenderWindow renderWindow = new RenderWindow(videoMode, title, style);
        renderWindow.Closed += Close;
        Window = renderWindow;
        BackgroundColor = color ?? BackgroundColor;
    }

    public virtual void Close(object? sender, EventArgs e)
    {
        (sender as RenderWindow)?.Close();
    }

    public virtual void DrawElements()
    {
        Window.DispatchEvents();
        Window.Clear(BackgroundColor);

        foreach (Drawable element in Elements)
        {
            if (element is Button button)
            {
                button.Draw(Window, RenderStates.Default);
                if (button.IsMouseDown(Window))
                {
                    button.Click();
                }
                else if (button.IsMouseOver(Window))
                {
                    button.Highlight();
                }
                else
                {
                    button.Reset();
                }
            }
            else
            {
                Window.Draw(element);
            }
        }

        Window.Display();
    }

    public virtual void Start()
    {

    }
}