package org.demo.protobuf;

import java.io.IOException;
import java.util.Arrays;

/**
 * @author egal
 * @since 2024/8/18
 */

public class ProtobufExample {

    public static void main(String[] args) {
        try {
            // 编码过程
            HelloRequest request = HelloRequest.newBuilder()
                    .setName("Alice")
                    .build();

            // 将消息序列化为字节数组
            byte[] encodedRequest = request.toByteArray();
            System.out.println("encodedRequest: " + Arrays.toString(encodedRequest));

            // 打印编码后的字节数组
            System.out.println("Encoded Request: " + java.util.Base64.getEncoder().encodeToString(encodedRequest));

            // 解码过程
            HelloRequest decodedRequest = HelloRequest.parseFrom(encodedRequest);
            System.out.println("Decoded Request Name: " + decodedRequest.getName());

            // 创建回复消息
            HelloReply reply = HelloReply.newBuilder()
                    .setMessage("Hello, " + decodedRequest.getName() + "!")
                    .build();

            // 将回复消息序列化
            byte[] encodedReply = reply.toByteArray();
            System.out.println("encodedReply: " + Arrays.toString(encodedReply));
            HelloReply decodedReply = HelloReply.parseFrom(encodedReply);
            System.out.println("Decoded Reply Message: " + decodedReply.getMessage());

        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
