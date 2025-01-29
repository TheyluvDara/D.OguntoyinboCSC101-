                      Table "public.project"
      Column       |     Type     | Collation | Nullable | Default 
-------------------+--------------+-----------+----------+---------
 pno               | integer      |           | not null | 
 pname             | character(1) |           | not null | 
 pduration         | text         |           | not null | 
 project_managerid | integer      |           | not null | 
Indexes:
    "project_pkey" PRIMARY KEY, btree (project_managerid)

