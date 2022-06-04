CREATE TABLE IF NOT EXISTS comments (
    id INT,
    userid INT NOT NULL,
    postid INT NOT NULL,
    comment TEXT NOT NULL,
    date VARCHAR(50) NOT NULL,
    likes INT NOT NULL,
    timestamp BIGINT NOT NULL,
    PRIMARY KEY (id)
);
