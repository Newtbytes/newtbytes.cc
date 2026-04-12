{% if get_env(name='CENSORED', default=false) == 'true' %}
    {%- if repl is defined %} {{ repl | safe }} {% endif -%}
    {# this '-' somehow prevents the replacement from turning into a codeblock when it isn't supposed to. I don't know how. Also the comment has to go after the line and not before because that also causes the same problem. #}
{% else %}
    {%- if orig is defined %} {{ orig | safe }} {% endif -%}
{% endif %}
