-- Your SQL goes here
CREATE TABLE "room"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"title" TEXT NOT NULL,
	"owner_id" TEXT NOT NULL
);

CREATE TABLE "message"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"content" TEXT NOT NULL,
	"user_id" TEXT NOT NULL,
	"time_sent" TIMESTAMP NOT NULL,
	"room_id" INT4 NOT NULL,
	FOREIGN KEY ("room_id") REFERENCES "room"("id")
);

