using Microsoft.Win32;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;

namespace MetalFatiguePatchProject
{
    /// <summary>
    /// This class contains the main window of the application.
    /// </summary>
    public partial class MainWindow : Window
    {
        /// <summary>
        /// Initializes a new instance of the MainWindow class.
        /// </summary>
        public MainWindow()
        {
            InitializeComponent();
        }

        /// <summary>
        /// This method is called when the Patch button is clicked. It patches the executable file.
        /// </summary>
        private void Patch_Click(object sender, RoutedEventArgs e)
        {
            string executablePath = Input_path_txt.Text;
            byte[] patchBytes = get_ram_error_byte_arr_fixed();
            Patch_File(executablePath, patchBytes);

            out_txt.Content = "   Successfully Patched";
        }

        /// <summary>
        /// This method is called when the Unpatch button is clicked. It unpatches the executable file.
        /// </summary>
        private void Unpatch_Click(object sender, RoutedEventArgs e)
        {
            string executablePath = Input_path_txt.Text;
            byte[] patchBytes = get_ram_error_byte_arr_original();
            Patch_File(executablePath, patchBytes);

            out_txt.Content = "   Successfully Unpatched";
        }

        /// <summary>
        /// This method patches the executable file.
        /// </summary>
        /// <param name="executablePath">The path of the executable file.</param>
        /// <param name="patchBytes">The bytes to be patched.</param>
        private static void Patch_File(string executablePath, byte[] patchBytes)
        {
            int offset = 0xD280; // Offset des patch bereiches

            // Lese den Inhalt der ausführbaren Datei
            byte[] fileBytes = File.ReadAllBytes(executablePath);

            // Führe eine Änderung im Hexadezimalformat durch
            Array.Copy(patchBytes, 0, fileBytes, offset, patchBytes.Length);

            // Speichere die geänderte Datei zurück
            File.WriteAllBytes(executablePath, fileBytes);
        }

        /// <summary>
        /// Byte String for patching RAM error.
        /// Search str: GlobalAlloc
        /// orig 0x1150000:
        /// 68 00 00 15 01 6A 00 FF 15 14 21 4D 00 85 C0 74 2F 8D 88 F4 FF 14 01 C7 41 08 D8 2D 52 00 C7 41 04 00 00 00 00 89 01 89 08 2B C8 C7 40 08 00 00
        /// new 0x22A0000:
        /// 68 00 00 2A 02 6A 00 FF 15 14 21 4D 00 85 C0 74 2F 8D 88 F4 FF 14 01 C7 41 08 D8 2D 52 00 C7 41 04 00 00 00 00 89 01 89 08 2B C8 C7 40 08 00 00
        /// </summary>
        private static byte[] get_ram_error_byte_arr_fixed()
            => new byte[] {0x68,
                           0x00,
                           0x00,
                           0x2A,
                           0x02,
                           0x6A,
                           0x00,
                           0xFF,
                           0x15,
                           0x14,
                           0x21,
                           0x4D,
                           0x00,
                           0x85,
                           (byte)0xC0,
                           (byte)0x74,
                           (byte)0x2F,
                           (byte)0x8D,
                           (byte)0x88,
                           (byte)0xF4,
                           (byte)0xFF,
                           (byte)0x14,
                           (byte)0x01,
                           (byte)0xC7,
                           (byte)0x41,
                           (byte)0x08,
                           (byte)0xD8,
                           (byte)0x2D,
                           (byte)0x52,
                           (byte)0x00,
                           (byte)0xC7,
                           (byte)0x41,
                           (byte)0x04,
                           (byte)0x00,
                           (byte)0x00,
                           (byte)0x00,
                           (byte)0x00,
                           (byte)0x89,
                           (byte)0x01,
                           (byte)0x89,
                           (byte)0x08,
                           (byte)0x2B,
                           (byte)0xC8,
                           (byte)0xC7,
                           (byte)0x40,
                           (byte)0x08,
                           (byte)0x00,
                           (byte)0x00
                    };
        /// <summary>
        /// Byte String for patching RAM error.
        /// Search str: GlobalAlloc
        /// orig 0x1150000:
        /// 68 00 00 15 01 6A 00 FF 15 14 21 4D 00 85 C0 74 2F 8D 88 F4 FF 14 01 C7 41 08 D8 2D 52 00 C7 41 04 00 00 00 00 89 01 89 08 2B C8 C7 40 08 00 00
        /// neu 0x22A0000:
        /// 68 00 00 2A 02 6A 00 FF 15 14 21 4D 00 85 C0 74 2F 8D 88 F4 FF 14 01 C7 41 08 D8 2D 52 00 C7 41 04 00 00 00 00 89 01 89 08 2B C8 C7 40 08 00 00
        /// </summary>
        private static byte[] get_ram_error_byte_arr_original()
        => new byte[] {0x68,
                           0x00,
                           0x00,
                           0x15,
                           0x01,
                           0x6A,
                           0x00,
                           0xFF,
                           0x15,
                           0x14,
                           0x21,
                           0x4D,
                           0x00,
                           0x85,
                           (byte)0xC0,
                           (byte)0x74,
                           (byte)0x2F,
                           (byte)0x8D,
                           (byte)0x88,
                           (byte)0xF4,
                           (byte)0xFF,
                           (byte)0x14,
                           (byte)0x01,
                           (byte)0xC7,
                           (byte)0x41,
                           (byte)0x08,
                           (byte)0xD8,
                           (byte)0x2D,
                           (byte)0x52,
                           (byte)0x00,
                           (byte)0xC7,
                           (byte)0x41,
                           (byte)0x04,
                           (byte)0x00,
                           (byte)0x00,
                           (byte)0x00,
                           (byte)0x00,
                           (byte)0x89,
                           (byte)0x01,
                           (byte)0x89,
                           (byte)0x08,
                           (byte)0x2B,
                           (byte)0xC8,
                           (byte)0xC7,
                           (byte)0x40,
                           (byte)0x08,
                           (byte)0x00,
                           (byte)0x00
            };

        private void Select_Path(object sender, RoutedEventArgs e)
        {
            OpenFileDialog openFileDialog = new OpenFileDialog();
            if (openFileDialog.ShowDialog() == true)
            {
                Input_path_txt.Text = openFileDialog.FileName;
            }

        }
    }
}
