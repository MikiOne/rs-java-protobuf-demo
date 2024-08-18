package org.demo.protobuf;

import org.demo.protobuf.generated.Student;

import java.io.IOException;
import java.io.OutputStream;
import java.net.Socket;
import java.nio.ByteBuffer;

/**
 * @author egal
 * @since 2024/8/18
 */

public class TcpProtobufClient {
    private static byte[] bytesStudent() {
        Student student = Student.newBuilder().setId(123).setFirstName("maxsm")
                .setLastName("sdfsdfsdfsdfsdfsdsdfsdfsdfsdfsdfsdfsdfsdf")
                .setEmail("1224sdfsfsdf344552@163.com")
                .addPhone(Student.PhoneNumber.newBuilder().setNumber("12345sdfsdfsd6566666").setType(
                        Student.PhoneType.MOBILE).build()).build();
        return student.toByteArray();
    }

    public static void main(String[] args) {
        String serverAddress = "127.0.0.1";
        int port = 9527;

        try (Socket socket = new Socket(serverAddress, port);
             OutputStream out = socket.getOutputStream()) {

            // 要发送的消息
            byte[] messageBytes = bytesStudent();;

            // 创建一个字节缓冲区，用于存储长度和消息
            ByteBuffer buffer = ByteBuffer.allocate(4 + messageBytes.length);
            buffer.putInt(messageBytes.length); // 添加消息长度
            buffer.put(messageBytes); // 添加消息内容

            // 发送数据
            out.write(buffer.array());
            out.flush();

            System.out.println("Message sent: " + messageBytes);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

//    public static void main(String[] args) {
//        try {
//            // 1. 创建Socket对象，连接到服务端
//            Socket socket = new Socket("localhost", 9527);
//
//            // 2. 获取Socket的输出流
//            OutputStream out = socket.getOutputStream();
//
//            // 3. 发送byte[]数组
//            byte[] bytes = bytesStudent();
//            out.write(bytes);
//            out.flush();
//
//            // 4. 关闭Socket
//            socket.close();
//        } catch (IOException e) {
//            e.printStackTrace();
//        }
//    }
//    public static void main(String[] args) {
//        String hostname = "localhost"; // 服务器地址
//        int port = 9527; // 服务器端口
//
//        try (Socket socket = new Socket(hostname, port)) {
//            // 设置输入输出流
//            PrintWriter out = new PrintWriter(socket.getOutputStream(), true);
//            BufferedReader in = new BufferedReader(new InputStreamReader(socket.getInputStream()));
//
//            // 发送消息到服务器
////            String message = "Hello, Server!";
////            out.println(message);
////            System.out.println("Sent: " + message);
//            byte[] bytes = bytesStudent();
//            out.print(bytes);
//
//            // 接收服务器的响应
//            String response = in.readLine();
//            System.out.println("Received: " + response);
//        } catch (IOException e) {
//            System.out.println("Client exception: " + e.getMessage());
//            e.printStackTrace();
//        }
//    }
}
