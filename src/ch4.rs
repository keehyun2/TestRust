pub fn chater4() {

    let mut s = String::from("hello");
    // let mut s = "hello";     
    // 리터럴 형식은 값을 바꿀수 없음. 리터럴은 스택에 저장됨

    s.push_str(", world!"); 
    // push_str()은 해당 스트링 리터럴을 스트링에 붙여줍니다.
    
    println!("{}", s);

}

