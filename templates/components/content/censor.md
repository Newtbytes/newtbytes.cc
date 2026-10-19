{% if get_env(name='CENSORED', default=false) == 'true' %}
    {%- if repl is defined %} {{ repl | safe }} {% endif -%}
{% else %}
    {%- if body is defined %}
        {{- body | safe -}}
    {% elif orig is defined %}
        {{- orig | safe -}}
    {% endif -%}
{% endif %}
