FROM clojure:onbuild as BUILD
COPY . /usr/src/app
WORKDIR /usr/src/app
RUN lein ring uberjar

FROM openjdk:8-jdk-alpine
ENV PORT 8080
EXPOSE 8080
COPY --from=BUILD /usr/src/app/target/hello-world-0.1.0-SNAPSHOT-standalone.jar /opt/app.jar
WORKDIR /opt
CMD ["java", "-jar", "app.jar"]
