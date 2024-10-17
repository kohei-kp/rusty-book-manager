INSERT INTO
    roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users (name, email, password_hash, role_id)
SELECT
    'Eleazar Fig',
    'eleazar.fig@example.com',
    '$2b$12$p157tNvZsZWaky.K/uYlYuT4AaSz7SNrpPVB7EYk.8YmH/N0wsuaC',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';
