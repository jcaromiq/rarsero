What's Rarsero?
===============
Rarsero is a free script written in Rust which reads the Robots.txt
file of a web server and looks at the Disallow entries. The Disallow
entries tell the search engines what directories or files hosted on a
web server mustn't be indexed. For example, "Disallow: /portal/login"
means that the content on www.example.com/portal/login it's not allowed
to be indexed by crawlers like Google, Bing, Yahoo... This is the way
the administrator have to not share sensitive or private information
with the search engines.

But sometimes these paths typed in the Disallows entries are directly
accessible by the users without using a search engine, just visiting
the URL and the Path, and sometimes they are not available to be visited
by anybody. Because it is really common that the administrators write
a lot of Disallows and some of them are available and some of them are
not, you can use Rarsero in order to check the HTTP status code of each
Disallow entry in order to check automatically if these directories are
available or not.

When you execute Rarsero, you can see the HTTP status codes. For example,
the codes bellow:

    200 OK          The request has succeeded.
    403 Forbidden   The server understood the request, but is refusing to fulfill it.
    404 Not Found   The server hasn't found anything matching the Request-URI.
    302 Found       The requested resource resides temporarily under a different URI.
    ...


Installing
==========
TBD

Usage
=====

    TBD

Example
=======
	 
    # Rarsero www.example.com

    

Disclaimer
==========
The use of this tool is your responsability. Use Rarsero to audit your
own servers or servers you are allowed to scan. I hereby disclaim any
responsibility for actions taken with this tool.

Author
======



Credits
=======
