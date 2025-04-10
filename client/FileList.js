import React, { useState, useEffect, useCallback } from 'react';
import axios from 'axios';

const UploadedFiles = () => {
    const [files, setFiles] = useState([]);

    const fetchFiles = useCallback(async () => {
        try {
            const response = await axios.get(`${process.env.REACT_APP_BACKEND_URL}/files`);
            setFiles(response.data);
        } catch (error) {
            console.error("Failed to fetch files:", error);
        }
    }, []);

    useEffect(() => {
        fetchFiles();
    }, [fetchFiles]);

    if (files.length === 0) {
        return <div><h2>Uploaded Files</h2><p>No files uploaded yet.</p></div>;
    }

    return (
        <div>
            <h2>Uploaded Files</h2>
            <ul>
                {files.map((file) => (
                    <li key={file.id || file.name}>{file.name}</li>
                ))}
            </ul>
        </div>
    );
};

export default UploadedFiles;