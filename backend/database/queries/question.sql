--! create
INSERT INTO questions(content)
VALUES(:content)
RETURNING id;

--! get_all
SELECT id, content
FROM questions
WHERE is_active = true;

--! update
UPDATE questions
SET content = :content
WHERE id = :id;

--! delete
UPDATE questions
SET is_active = false
WHERE id = :id;
