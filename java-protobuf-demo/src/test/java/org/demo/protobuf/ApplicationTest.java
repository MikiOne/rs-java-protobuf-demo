package org.demo.protobuf;

/**
 * @author egal
 * @since 2024/8/18
 */

import org.demo.protobuf.generated.Student;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.http.ResponseEntity;
import org.springframework.web.client.RestTemplate;

@SpringBootTest(classes = DemoApplication.class)
public class ApplicationTest {
    // Other declarations
    private static final String COURSE1_URL = "http://localhost:8088/protobuf/1";
    private static final String COURSE2_URL = "http://localhost:9527";

    @Autowired
    private RestTemplate restTemplate;

    @Test
    void whenUsingRestTemplate_thenSucceed() {
        ResponseEntity<Student> student = restTemplate.getForEntity(COURSE1_URL, Student.class);
        System.out.println(student);
    }

}
