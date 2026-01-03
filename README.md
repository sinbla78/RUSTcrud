# Rust Todo 서비스

Rust와 Warp 프레임워크로 구축한 간단하고 빠른 Todo 관리 REST API 서버입니다.

## 주요 기능

- **CRUD 작업**: Todo 생성, 조회, 수정, 삭제
- **RESTful API**: 직관적인 HTTP 엔드포인트
- **메모리 저장소**: 빠른 인메모리 데이터 저장
- **CORS 지원**: 웹 애플리케이션과의 원활한 통합
- **구조화된 설계**: 모듈별로 분리된 깔끔한 코드 구조
- **에러 처리**: 체계적인 에러 핸들링 및 응답

## 빠른 시작

### 1. 사전 요구사항

Rust가 설치되어 있어야 합니다. [rustup.rs](https://rustup.rs/)에서 설치할 수 있습니다.

```bash
# Rust 설치 (macOS/Linux)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 설치 확인
rustc --version
cargo --version
```

### 2. 프로젝트 클론 및 실행

```bash
# 프로젝트 클론
git clone https://github.com/username/RUSTcrud.git
cd RUSTcrud

# 의존성 설치 및 실행
cargo run
```

서버가 `http://127.0.0.1:3030`에서 실행됩니다.

### 3. 헬스 체크

```bash
curl http://localhost:3030/health
```

## 📚 API 문서

### 기본 URL
```
http://localhost:3030
```

### 엔드포인트

| 메서드 | 경로 | 설명 | 요청 본문 |
|--------|------|------|-----------|
| `GET` | `/health` | 서버 상태 확인 | - |
| `GET` | `/todos` | 모든 할 일 조회 | - |
| `POST` | `/todos` | 새 할 일 생성 | `{"title": "string"}` |
| `GET` | `/todos/{id}` | 특정 할 일 조회 | - |
| `PUT` | `/todos/{id}` | 할 일 수정 | `{"title": "string", "completed": boolean}` |
| `DELETE` | `/todos/{id}` | 할 일 삭제 | - |

### 예시 요청

#### 할 일 생성
```bash
curl -X POST http://localhost:3030/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Rust 공부하기"}'
```

#### 모든 할 일 조회
```bash
curl http://localhost:3030/todos
```

#### 할 일 완료 처리
```bash
curl -X PUT http://localhost:3030/todos/{todo_id} \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'
```

#### 할 일 삭제
```bash
curl -X DELETE http://localhost:3030/todos/{todo_id}
```

## 🏗️ 프로젝트 구조

```
src/
├── main.rs          # 애플리케이션 진입점
├── lib.rs           # 라이브러리 모듈 정의
├── config.rs        # 서버 설정 관리
├── models.rs        # 데이터 모델 및 DTO
├── store.rs         # 데이터 저장소 추상화
├── handlers.rs      # 비즈니스 로직 처리
├── routes.rs        # HTTP 라우팅
└── error.rs         # 에러 처리
```

## 🛠️ 사용 기술

- **[Rust](https://www.rust-lang.org/)**: 시스템 프로그래밍 언어
- **[Warp](https://github.com/seanmonstar/warp)**: 빠르고 안전한 웹 프레임워크
- **[Tokio](https://tokio.rs/)**: 비동기 런타임
- **[Serde](https://serde.rs/)**: JSON 직렬화/역직렬화
- **[UUID](https://github.com/uuid-rs/uuid)**: 고유 식별자 생성
- **[Tracing](https://tracing.rs/)**: 구조화된 로깅

## 🧪 테스트

```bash
# 단위 테스트 실행
cargo test

# 코드 검사
cargo check

# 코드 포맷팅
cargo fmt

# 린팅
cargo clippy
```

## 🔧 개발 도구

### 개발 모드로 실행 (자동 재시작)
```bash
# cargo-watch 설치
cargo install cargo-watch

# 파일 변경 시 자동 재시작
cargo watch -x run
```

### 릴리즈 빌드
```bash
cargo build --release
./target/release/todo_service
```

## 🚀 확장 가능성

이 프로젝트는 다음과 같은 기능들로 확장할 수 있습니다:

- **데이터베이스 연동**: PostgreSQL, MongoDB 등
- **인증/인가**: JWT 토큰 기반 사용자 인증
- **API 문서화**: OpenAPI/Swagger 통합
- **캐싱**: Redis를 통한 성능 최적화
- **배포**: Docker 컨테이너화 및 클라우드 배포
- **모니터링**: 메트릭 수집 및 로그 분석

## 🤝 기여하기

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 라이센스

이 프로젝트는 MIT 라이센스 하에 배포됩니다. 자세한 내용은 `LICENSE` 파일을 참조하세요.

## 📞 문의

문제가 있거나 질문이 있으시면 이슈를 생성해 주세요.
