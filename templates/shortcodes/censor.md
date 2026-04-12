{% if get_env(name='CENSORED', default=false) == 'true' %}
    {%- if repl is defined %} {{ repl | safe }} {% endif -%}
{% else %}
    {%- if orig is defined %} {{ orig | safe }} {% endif -%}
{% endif %}
