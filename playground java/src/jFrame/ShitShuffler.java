package jFrame;

import java.awt.*;
import java.util.Arrays;

import javax.swing.*;

public class ShitShuffler {
    static int[] result;
    static int length;

    public static void init() {
        JTextField input = new JTextField("length: ");
        JButton submit = new JButton();
        JFrame f = new JFrame();
        JLabel out = new JLabel("{...}");
        f.setLayout(null);
        f.setSize(250, 300);
        f.setVisible(true);
        f.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        f.setResizable(false);

        input.setBounds(0, 0, 100, 40);

        submit.setBounds(0, 40, 100, 40);
        submit.addActionListener(e -> {
            try {
                int inp = Integer.parseInt(input.getText());
                if (inp <= 20)
                    out.setText(Arrays.toString(rad.ShitShuffler.run(inp)));
                else
                    out.setText("cant roll length over 20");
            } catch (Exception ex) {
                System.out.println(ex.getLocalizedMessage());
            }
        });
        submit.setText("submit");

        out.setBounds(0, 70, 150, 40);
        out.setVisible(true);
        f.add(submit);
        f.add(input);
        f.add(out);
    }
    
}
