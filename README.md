# Reflection

### 1. What are the key differences between unary, server streaming, and bi-directional streaming RPC (Remote Procedure Call) methods, and in what scenarios would each be most suitable?
- Unary adalah metode komunikasi dimana client mengirim single request dan server akan meresponse request tersebut dalam single response juga. Metode unary cocok untuk jika ingin mengambil 1 item saja dari database seperti autentikasi user. Jika unary server hanya mengirim 1 response, maka pada server streaming server mengirim stream of response yang menjadikan metode komunikasi ini cocok untuk skenario dimana server perlu mengirim data yang besar dan banyak ke client atau aliran update yang terus menerus ke client seperti info realtime (stock market price). bi-directional streaming memungkinkan server dan client juga sama-sama mengirim data stream satu sama lain sehingga komunikasi 2 arah secara realtime memungkinkan. skenario yang cocok adalah dimana client dan server perlu komunikasi 2 arah secara realtime seperti chat.

### 2. What are the potential security considerations involved in implementing a gRPC service in Rust, particularly regarding authentication, authorization, and data encryption?
- enkripsi data, authentikasi, dan otorisasi

### 3. What are the potential challenges or issues that may arise when handling bidirectional streaming in Rust gRPC, especially in scenarios like chat applications?
- mungkin penanganan koneksi disconnect dimana jika koneksi terputus tetapi server gagal membersihkan channel maka memungkinkan terjadi memory leak. Karena isunya juga sangat-sangat umum jadi makin tinggi prioritasnya.

### 4. What are the advantages and disadvantages of using the tokio_stream::wrappers::ReceiverStream for streaming responses in Rust gRPC services?
- salah satu kelebihannya adalah bisa mengubah mpsc menjadi format stream secara simpel tapi dilain sisi ketika background apps mengalami failure dan aliran stream putus, maka klien mungkin tidak mendapatkan pesan error yang spesifik

### 5. In what ways could the Rust gRPC code be structured to facilitate code reuse and modularity, promoting maintainability and extensibility over time?
- gunakan SOLID principle dan clean code principle. Lalu gunakan traits milik rust untuk dependency injection sehingga modul yang kita buat loosely coupled dan memudahkan developer untuk melakukan testing nantinya. 

### 6. In the MyPaymentService implementation, what additional steps might be necessary to handle more complex payment processing logic?
- mungkin input validation untuk memastikan parameter request valid sebelum di proses. payment processing yang complex tapi  disertai dengan input validation bisa meningkatkan keamanan dan mungkin performa code. Selain itu mungkin integrasi dengan database juga bisa dipertimbangkan untuk penyimpanan data.

### 7. What impact does the adoption of gRPC as a communication protocol have on the overall architecture and design of distributed systems, particularly in terms of interoperability with other technologies and platforms?
- gRPC menggunakan HTTP/2 dan format biner protobuf yang dimana membuat app kita berjalan lebih efisien dan berperforma lebih baik. Lalu dari sisi interoperabilitas juga gRPC unggul dalam komunikasi backend yang menggunakan programming language yang berbeda-beda. overall gRPC adalah sistem modern yang otomatis pasti futureproof untuk membangun arsitektur baru. namun karena support browser belum seluas REST, maka gRPC harus disesuaikan use casenya dengan REST.

### 8. What are the advantages and disadvantages of using HTTP/2, the underlying protocol for gRPC, compared to HTTP/1.1 or HTTP/1.1 with WebSocket for REST APIs?
- HTTP/2 memungkinkan kita mengirim banyak request dan response secara bersamaan secara asinkron sehingga memnjawab permasalahan HTTP/1.1 yang harus berjalan secara sequential. Selain itu HTTP/2 juga mengompresi header pada sisi client dan server sehingga mengurangi payload. Namun HTTP/2 juga memiliki kekurangan yaitu sulit di debug karena data HTTP/2 tidak lagi berbentuk plain text. Lalu juga HTTP/2 menuntut daya komputasi CPU yang lebih besar dari HTTP/1.1 karena manajemen stream yang banyak pada 1 koneksi dan lainnya.

### 9. How does the request-response model of REST APIs contrast with the bidirectional streaming capabilities of gRPC in terms of real-time communication and responsiveness?
- REST API bekerja dengan prinsip unary dimana client harus mengirim request dulu baru server bisa merespon balik. karena bekerjanya harus saling menunggu seperti inilah unary kurang cocok untuk komunikasi realtime dan pekerjaan lainnya yang menuntuk responsivitas. Berbeda dengan REST, gRPC menggunakan bidirectional yang memungkinkan server dan juga client masing-masing bisa mengirimkan pesan secara bolak-balik tanpa perlu menunggu (berjalan mandiri) dalam stream. Hal ini memungkinkan pekerjaan yang menuntun responsivitas dan realtime komunikasi cocok berjalan disini. server tidak perlu menunggu client dan client juga tidak perlu menunggu server.


### 10. What are the implications of the schema-based approach of gRPC, using Protocol Buffers, compared to the more flexible, schema-less nature of JSON in REST API payloads?
- protobuf lebih ketat validasi pesannya dengan definisi skema yang dilakukan sehingga tipe data bisa selalu konsisten. REST APi dilain sisi menggunakan JSON yang tidak memiliki skema bawaan yang baku sehingga proses data JSON harus divalidasi lebih lanjut untuk memastikan isi dan struktur dari datanya sudah benar.

