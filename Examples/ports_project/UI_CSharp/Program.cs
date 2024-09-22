using SFML.Graphics;
using SFML.System;
using SFML.Window;
using SFML_Game_Loop;

namespace UI_CSharp
{
    static class Program
    {
        public static void Main()
        {
            MainWindow window = new();
            window.BeginRoutine();
        }
    }
    public class MainWindow : ModifiedWindowRoutine
    {
        public const string ports_directory = "D:\\VirtualComputer\\VC\\Examples\\ports_project\\rust_program\\src\\ports\\";
        public RectangleShape[,] PixelMatrix = new RectangleShape[120,120];
        public RectangleShape? Trackpad;
        public Text[]? Ports;
        public Text? ErrorText;
        public (Vector2i current, Vector2i last) track;
        public MainWindow() : base(new(805, 820), "rust ports project", Styles.Close, new(70, 80, 90)) { }

        public override void Start()
        {
            RectangleShape screen = new()
            {
                Position = new(25, 25),
                Size = new(240, 240),
                FillColor = Color.Black
            };
            RectangleShape computer = new()
            {
                Position = new(25, 290),
                Size = new(240, 485),
                FillColor = new(40, 40, 40)
            };
            RectangleShape mouse_pad = new()
            {
                Position = new(295, 25),
                Size = new(485, 240),
                FillColor = new(30, 30, 30)
            };
            RectangleShape keyboard = new()
            {
                Position = new(295, 290),
                Size = new(485, 485),
                FillColor = new(10, 10, 10)
            };
            Text mouse_pad_text = new()
            {
                Position = new(465, 125),
                DisplayedString = "Mouse Pad",
                Font = new(DefaultFontPath),
                FillColor = Color.White
            };
            Text computer_text = new()
            {
                Position = new(72, 305),
                DisplayedString = "Computer",
                Font = new(DefaultFontPath),
                FillColor = Color.White,
            };

            Text port0_text = new()
            {
                Position = new(47, 360),
                DisplayedString = "PORT 0: 00000000",
                Font = new(DefaultFontPath),
                FillColor = new(245, 245, 245),
                CharacterSize = 24
            };
            Text port1_text = new()
            {
                Position = new(47, 400),
                DisplayedString = "PORT 1: 00000000",
                Font = new(DefaultFontPath),
                FillColor = new(245, 245, 245),
                CharacterSize = 24
            };
            Text port2_text = new()
            {
                Position = new(47, 440),
                DisplayedString = "PORT 2: 00000000",
                Font = new(DefaultFontPath),
                FillColor = new(245, 245, 245),
                CharacterSize = 24
            };
            Text port3_text = new()
            {
                Position = new(47, 480),
                DisplayedString = "PORT 3: 00000000",
                Font = new(DefaultFontPath),
                FillColor = new(245, 245, 245),
                CharacterSize = 24
            };
            Text port4_text = new()
            {
                Position = new(47, 520),
                DisplayedString = "PORT 4: 00000000",
                Font = new(DefaultFontPath),
                FillColor = new(245, 245, 245),
                CharacterSize = 24
            };
            Text port5_text = new()
            {
                Position = new(47, 560),
                DisplayedString = "PORT 5: 00000000",
                Font = new(DefaultFontPath),
                FillColor = new(245, 245, 245),
                CharacterSize = 24
            };
            Text port6_text = new()
            {
                Position = new(47, 600),
                DisplayedString = "PORT 6: 00000000",
                Font = new(DefaultFontPath),
                FillColor = new(245, 245, 245),
                CharacterSize = 24
            };
            Text port7_text = new()
            {
                Position = new(47, 640),
                DisplayedString = "PORT 7: 00000000",
                Font = new(DefaultFontPath),
                FillColor = new(245, 245, 245),
                CharacterSize = 24
            };
            ErrorText = new()
            {
                Position = new(20, 790),
                DisplayedString = "",
                Font = new(DefaultFontPath),
                FillColor = new(250, 35, 35),
                CharacterSize = 14
            };

            var keys = DrawKeys();
            var pixels = Pixels();
            Ports = [port0_text, port1_text, port2_text, port3_text, port4_text, port5_text, port6_text, port7_text];
            Trackpad = mouse_pad;

            Elements.AddRange([screen, computer, mouse_pad, keyboard, mouse_pad_text, computer_text, ErrorText, .. Ports, .. keys, .. pixels]);
        }
        public bool IsMouseOverTrackpad()
        {
            Vector2i position = Mouse.GetPosition(Window);
            return Trackpad!.GetGlobalBounds().Contains(position.X, position.Y);
        }

        public bool IsMouseDownTrackpad()
        {
            if (IsMouseOverTrackpad())
            {
                if (Mouse.IsButtonPressed(Mouse.Button.Left))
                {
                    track.last = track.current;
                    track.current = Mouse.GetPosition();
                    return true;
                }
            }
            return false;
        }

        public Drawable[] Pixels()
        {
            Drawable[] pixels = [];
            for (int x = 0; x < 120; x++)
            {
                for (int y = 0; y < 120; y++)
                {
                    var pixel = GetPixel(new(x, y));
                    pixels = [.. pixels, pixel];
                    PixelMatrix[x, y] = pixel;
                }
            }
            return pixels;
        }
        public static RectangleShape GetPixel((int, int) pos)
        {
            return new()
            {
                Position = new(25 + (pos.Item1 * 2), 25 + (pos.Item2 * 2)),
                Size = new(2, 2),
                FillColor = Color.Black
            };
        }

        public void BeginRoutine()
        {
            Start();

            while (Window.IsOpen)
            {
                DrawElements();
                Update();
            }
        }
        
        public void Update()
        {
            try
            {
                if (Keyboard.IsKeyPressed(Keyboard.Key.Escape))
                {
                    Window.Close();
                }
                string[] ports = [
                    File.ReadAllText(Path.Combine(ports_directory, "0")), // pixel X 
                    File.ReadAllText(Path.Combine(ports_directory, "1")), // pixel Y
                    File.ReadAllText(Path.Combine(ports_directory, "2")), // red
                    File.ReadAllText(Path.Combine(ports_directory, "3")), // green
                    File.ReadAllText(Path.Combine(ports_directory, "4")), // blue
                    File.ReadAllText(Path.Combine(ports_directory, "5")), // track pad x
                    File.ReadAllText(Path.Combine(ports_directory, "6")), // track pad y
                    File.ReadAllText(Path.Combine(ports_directory, "7")), // keyboard
                ];
                var pixel_x = Convert.ToInt32(ports[0], 2);
                var pixel_y = Convert.ToInt32(ports[1], 2);

                var red = Convert.ToInt32(ports[2], 2);
                var green = Convert.ToInt32(ports[3], 2);
                var blue = Convert.ToInt32(ports[4], 2);
                var color = new Color((byte)red, (byte)green, (byte)blue);

                PixelMatrix[pixel_x, pixel_y].FillColor = color;

                if (IsMouseDownTrackpad() && !track.last.Equals(new(0, 0)))
                {
                    var difference = track.last - track.current;
                    var difference_clamped = new Vector2i(Math.Clamp(difference.X, -255, 255), Math.Clamp(difference.Y, -255, 255));
                    var binary_x = Convert.ToString(Math.Abs(difference_clamped.X), 2);
                    var binary_y = Convert.ToString(Math.Abs(difference_clamped.Y), 2);

                    var binary_x_fmt = new string('0', 8 - binary_x.Length) + binary_x;
                    var binary_y_fmt = new string('0', 8 - binary_y.Length) + binary_y;

                    File.WriteAllText(Path.Combine(ports_directory, "5"), binary_x_fmt);
                    File.WriteAllText(Path.Combine(ports_directory, "6"), binary_y_fmt);
                }
                for (int i = 0; i < Ports!.Length; i++)
                {
                    if (i == 5 || i == 6)
                    {
                        Ports[i].DisplayedString = $"PORT {i}: {Convert.ToInt32(ports[i], 2)}";
                    }
                    else
                    {
                        var binary = File.ReadAllText(Path.Combine(ports_directory, i.ToString()));
                        Ports[i].DisplayedString = $"PORT {i}: {Convert.ToInt32(binary, 2)}";
                    }
                }
            }
            catch (Exception e)
            {
                ErrorText!.DisplayedString = e.Message;
            }
        }
        public void KeyboardClicked(string key)
        {
            try
            {
                var k = key switch
                {
                    "SPACE" => ' ',
                    "ENTER" => '\r',
                    "TAB" => '\t',
                    "UP" => '^',
                    "DOWN" => '\'',
                    "LEFT" => '<',
                    "RIGHT" => '>',
                    _ => key.ToCharArray()[0]
                };
                var binary = Convert.ToString(k, 2);
                var binary_fmt = new string('0', 8 - binary.Length) + binary;

                File.WriteAllText(Path.Combine(ports_directory, "7"), binary_fmt);
            }
            catch (Exception e)
            {
                ErrorText!.DisplayedString = e.Message;
            }
        }
        public Drawable[] DrawKeys()
        {
            return [
                KeyboardKey("A", new(320, 320)),
                KeyboardKey("B", new(375, 320)),
                KeyboardKey("C", new(430, 320)),
                KeyboardKey("D", new(485, 320)),
                KeyboardKey("E", new(540, 320)),
                KeyboardKey("F", new(595, 320)),
                KeyboardKey("G", new(650, 320)),
                KeyboardKey("H", new(705, 320)),

                KeyboardKey("I", new(320, 375)),
                KeyboardKey("J", new(375, 375)),
                KeyboardKey("K", new(430, 375)),
                KeyboardKey("L", new(485, 375)),
                KeyboardKey("M", new(540, 375)),
                KeyboardKey("N", new(595, 375)),
                KeyboardKey("O", new(650, 375)),
                KeyboardKey("P", new(705, 375)),

                KeyboardKey("Q", new(320, 430)),
                KeyboardKey("R", new(375, 430)),
                KeyboardKey("S", new(430, 430)),
                KeyboardKey("T", new(485, 430)),
                KeyboardKey("U", new(540, 430)),
                KeyboardKey("V", new(595, 430)),
                KeyboardKey("W", new(650, 430)),
                KeyboardKey("X", new(705, 430)),

                KeyboardKey("Y", new(320, 485)),
                KeyboardKey("Z", new(375, 485)),
                KeyboardKey("0", new(430, 485)),
                KeyboardKey("1", new(485, 485)),
                KeyboardKey("2", new(540, 485)),
                KeyboardKey("3", new(595, 485)),
                KeyboardKey("4", new(650, 485)),
                KeyboardKey("5", new(705, 485)),

                KeyboardKey("6", new(320, 540)),
                KeyboardKey("7", new(375, 540)),
                KeyboardKey("8", new(430, 540)),
                KeyboardKey("9", new(485, 540)),
                KeyboardKey(".", new(540, 540)),
                KeyboardKey(",", new(595, 540)),
                KeyboardKey("!", new(650, 540)),
                KeyboardKey("?", new(705, 540)),

                KeyboardKey("@", new(320, 595)),
                KeyboardKey("#", new(375, 595)),
                KeyboardKey("$", new(430, 595)),
                KeyboardKey("&", new(485, 595)),
                KeyboardKey("+", new(540, 595)),
                KeyboardKey("-", new(595, 595)),
                KeyboardKey("*", new(650, 595)),
                KeyboardKey("/", new(705, 595)),

                KeyboardKey("ENTER", new(320, 650), new(105, 50)),
                KeyboardKey("TAB", new(430, 650), new(105, 50)),
                KeyboardKey("UP", new(540, 650)),
                KeyboardKey("DOWN", new(595, 650)),
                KeyboardKey("LEFT", new(650, 650)),
                KeyboardKey("RIGHT", new(705, 650)),

                KeyboardKey("SPACE", new(320, 705), new(435, 50)),
                ];
        }
        public Drawable KeyboardKey(string name, Vector2f position, Vector2f? size = null)
        {
            Button key = new(
                position: position,
                size: size ?? new(50, 50),
                color: new(15, 15, 15),
                fontColor: new(250, 250, 250),
                text: name,
                charSize: 16
            )
            {
                HighlightColor = new(30, 30, 30),
                ClickColor = new(35, 35, 35),
            };
            key.Clicked += (_, _) => KeyboardClicked(name);
            return key;
        }
    }
}