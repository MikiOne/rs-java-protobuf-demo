package org.demo.protobuf;

/**
 * @author egal
 * @since 2024/8/18
 */

import org.demo.protobuf.generated.Student;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.ArrayList;
import java.util.List;

@RestController
public class HelloWorldController {
    @RequestMapping("/json/{id}")
    public StudentJson showHelloWorld(@PathVariable Integer id) {
        StudentJson studentJson = new StudentJson();
        studentJson.setId(id);
        studentJson.setFirstName("maxsm");
        studentJson.setLastName("sdfsdfsdfsdfsdfsdsdfsdfsdfsdfsdfsdfsdfsdf");
        studentJson.setEmail("1224sdfsfsdf344552@163.com");
        PhoneNumJson phoneNumJson = new PhoneNumJson();
        phoneNumJson.setNumber("12345sdfsdfsd6566666");
        phoneNumJson.setType(1);
        List<PhoneNumJson> list = new ArrayList<>();
        list.add(phoneNumJson);
        studentJson.setPhoneNumList(list);
        return studentJson;
    }

    @RequestMapping("/protobuf/{id}")
    Student protobuf(@PathVariable Integer id) {
        return Student.newBuilder().setId(id).setFirstName("maxsm")
                .setLastName("sdfsdfsdfsdfsdfsdsdfsdfsdfsdfsdfsdfsdfsdf")
                .setEmail("1224sdfsfsdf344552@163.com")
                .addPhone(Student.PhoneNumber.newBuilder().setNumber("12345sdfsdfsd6566666").setType(
                        Student.PhoneType.MOBILE).build()).build();
    }

}
