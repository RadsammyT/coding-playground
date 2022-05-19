package jFrame;
import javax.swing.*;

public class test {
    
    public static int init() {
        // frame
        JFrame f = new JFrame();
        f.setLayout(null);
        f.setVisible(true);
        f.setSize(500, 500);
        f.setDefaultCloseOperation(WindowConstants.EXIT_ON_CLOSE);
        // test button, located 100,100. size 50 x 100.
        // prints "test" on click
        JButton b = new JButton();
        b.setText("pee");
        f.add(b);
        b.setVisible(true);
        b.setBounds(100, 100, 50, 100);
        b.addActionListener(e -> {
            System.out.println("test");
        });

        
        return 0;
    }

}
