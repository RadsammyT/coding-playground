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
        JLabel fail = new JLabel("FAILS: ");
        
        f.setLayout(null);
        f.setSize(260, 150);
        f.setVisible(true);
        f.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        f.setResizable(false);
        f.setTitle("test");

        input.setBounds(0, 0, 100, 40);

        submit.setBounds(0, 40, 100, 40);
        submit.addActionListener(e -> {
            try {
                
                int inp = Integer.parseInt(input.getText());
                if (inp <= 20) {
                    int[] copy = rad.ShitShuffler.run(inp, true);
                    // out.setText(Arrays.toString(rad.ShitShuffler.run(inp, true)));
                    out.setText(Arrays.toString(Arrays.copyOfRange(copy, 0, inp)));
                    fail.setText("FAILS: " + Integer.toString(copy[inp]));

                } else
                    out.setText("cant roll length over 20");
            } catch (Exception ex) {
                System.out.println(ex.getLocalizedMessage());
                ex.printStackTrace();
            }
        });
        submit.setText("submit");

        out.setBounds(0, 70, 250, 40);
        out.setVisible(true);
        fail.setBounds(110, 50, 200, 40);
        fail.setVisible(true);
        f.add(submit);
        f.add(input);
        f.add(out);
        f.add(fail);
    }
    
}
