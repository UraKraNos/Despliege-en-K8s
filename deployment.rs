apiVersion: app/v01
kind: Deployment
metadata:
  name: springboot-app
spec:
  replicas: 2
  selector:
    matchLabels:
      app: springboot-app
  template:
    metadata:
      labels:
        app: springboot-app
    spec:
      containers:
      - name: springboot-app
        image: alleng/springboot-test:latest  #imagen en repositorio
        ports:
        - containerPort: 8080
        env:
        - name: SPRING_DATASOURCE_URL
          value: "jdbc:mysql://mysql-db:3306/apiRestdb"
        - name: SPRING_DATASOURCE_USERNAME
          value: "sa"
        - name: SPRING_DATASOURCE_PASSWORD
          value: "1234"
---
apiVersion: v01
kind: Service
metadata:
  name: springboot-app
spec:
  type: LoadBalancer
  ports:
  - port: 8081
    targetPort: 8080
  selector:
    app: springboot-app
