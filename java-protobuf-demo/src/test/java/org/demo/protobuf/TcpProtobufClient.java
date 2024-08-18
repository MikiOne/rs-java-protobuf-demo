package org.demo.protobuf;

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

}
